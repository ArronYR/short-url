use actix_files::Files;
use actix_web::{
    get, http, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use chrono::NaiveDateTime;
use log::{debug, error, info, warn};
use moka::future::Cache;
use sea_orm::{ConnectionTrait, Database, DbConn};
use short_url::models::link::{GenerateReq, Model as LinkModel, SearchParams};
use short_url::service::link::LinkService;
use short_url::utils::helpers::{generate_short_id, is_valid_url};
use short_url::{AppState, Config};
use std::env;
use std::time::Duration;
use tera::Tera;

const SCHEMA: &str = include_str!("../db.sql");
const DB_OPTION: &str = "characterEncoding=utf-8&characterSetResults=utf-8&autoReconnect=true&failOverReadOnly=false&serverTimezone=GMT%2B8";
const TOKEN: &str = "53ROYinHId9qke";

// 初始化配置信息
fn init_config() -> Config {
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let origin = env::var("ORIGIN").unwrap_or_else(|_| "https://s.nstp.cn".to_string());
    // mysql://username:password@host:port/database_name
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());
    let db_username = env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "root".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "short_url".to_string());
    let db_option = env::var("DB_OPTION").unwrap_or_else(|_| DB_OPTION.to_string());
    let token = env::var("TOKEN").unwrap_or_else(|_| TOKEN.to_string());
    let cache_max_cap = match env::var("CACHE_MAX_CAP") {
        Ok(value) => value.parse::<u64>().unwrap_or_else(|_| 1000),
        Err(_) => 1000,
    };
    let cache_live_time = match env::var("CACHE_LIVE_TIME") {
        Ok(value) => value.parse::<u64>().unwrap_or_else(|_| 60),
        Err(_) => 60,
    };

    Config {
        port,
        origin,
        db_host,
        db_port,
        db_username,
        db_password,
        db_name,
        db_option,
        token,
        cache_max_cap,
        cache_live_time,
    }
}

async fn build_error_page(request: HttpRequest, state: web::Data<AppState>) -> String {
    let template = &state.templates;

    let mut ctx = tera::Context::new();
    ctx.insert("uri", request.uri().path());

    template
        .render("error/404.html.tera", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
        .unwrap_or_else(|_| "error".to_string())
}

// 404
async fn not_found(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(build_error_page(request, state).await)
}

// 创建短链接的处理函数
#[post("/gen")]
async fn generate_short_link(
    request: HttpRequest,
    state: web::Data<AppState>,
    params: web::Json<GenerateReq>,
) -> impl Responder {
    let db_conn = &state.db_conn;
    let config = &state.config;

    let token = match request.headers().get("Token") {
        Some(header_value) => match header_value.to_str() {
            Ok(value) => value.to_string(),
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    };

    // 检查token是否正确
    if token != config.token {
        return HttpResponse::BadRequest().body("token不合法，请提供正确的参数");
    }
    // 检查URL是否合法
    if !is_valid_url(params.url.as_str()) {
        return HttpResponse::BadRequest().body("url不合法，请提供正确的参数");
    }
    // 如果源链接key已存在，直接返回
    if let Some(link) = LinkService::find_by_original_url(&db_conn, params.url.clone()).await {
        let short_link = format!("{origin}/{id}", origin = config.origin, id = link.short_id);
        return HttpResponse::Ok().body(short_link);
    };
    let mut short_id = generate_short_id();
    // 尝试3次，如果都已经存在，则抛出错误
    for i in 0..3 {
        if LinkService::check_short_id_used(&db_conn, short_id.clone()).await {
            // 已被使用则重新生成
            short_id = generate_short_id();
        } else {
            break;
        }
        if i == 2 {
            return HttpResponse::InternalServerError().body("暂时无法生成短链接，请稍后重试");
        }
    }
    let model = LinkModel {
        id: 0,
        short_id: short_id.clone(),
        original_url: params.url.clone(),
        create_time: NaiveDateTime::default(),
    };
    let result = LinkService::create(&db_conn, model).await;
    match result {
        Ok(_) => {
            let short_link = format!("{origin}/{id}", origin = config.origin, id = short_id);
            HttpResponse::Ok().body(short_link)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// 重定向短链接到原始链接的处理函数
#[get("/{short_id}")]
async fn redirect(
    request: HttpRequest,
    state: web::Data<AppState>,
    short_id: web::Path<String>,
) -> impl Responder {
    let db_conn = &state.db_conn;
    let cache = &state.cache;

    // 如果缓存存在
    if let Some(cached) = cache.get(short_id.as_str()).await {
        if let Some(url) = cached {
            info!(
                "[redirect] cached short_id: {} original_url: {}",
                short_id, url
            );
            return HttpResponse::TemporaryRedirect()
                .append_header(("Location", url))
                .finish();
        }
        warn!("[redirect] cached short_id: {} url invalid", short_id);
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(build_error_page(request, state).await);
    }
    let result = LinkService::find_by_short_id(&db_conn, short_id.to_string()).await;
    if let Some(row) = result {
        if row.original_url.is_empty() {
            error!("[redirect] short_id: {} original_url is empty", short_id);
            cache.insert(short_id.clone(), None).await;
            HttpResponse::NotFound().finish()
        } else {
            info!(
                "[redirect] short_id: {} original_url: {}",
                short_id, row.original_url
            );
            let original_url: String = row.original_url;
            cache
                .insert(short_id.clone(), Some(original_url.clone()))
                .await;
            HttpResponse::TemporaryRedirect()
                .append_header(("Location", original_url))
                .finish()
        }
    } else {
        error!("[redirect] short_id: {} is not found", short_id);
        cache.insert(short_id.clone(), None).await;
        HttpResponse::Ok()
            .content_type("text/html")
            .body(build_error_page(request, state).await)
    }
}

#[get("/")]
async fn home(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let db_conn = &state.db_conn;
    let config = &state.config;
    let template = &state.templates;

    // get params
    let params = web::Query::<SearchParams>::from_query(req.query_string()).unwrap();
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(30);

    let params = SearchParams {
        id: None,
        url: None,
        page: Some(page),
        size: Some(size),
    };

    let (links, pages) = LinkService::search(&db_conn, params)
        .await
        .expect("Cannot find links in page");

    let mut ctx = tera::Context::new();
    ctx.insert("origin", config.origin.as_str());
    ctx.insert("page", &page);
    ctx.insert("size", &size);
    ctx.insert("links", &links);
    ctx.insert("pages", &pages);

    let body = template
        .render("index.html.tera", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

// 初始化数据库表结构
async fn init_db(conn: DbConn) {
    conn.execute_unprepared(SCHEMA)
        .await
        .expect("init db error");
}

async fn init_db_conn(config: Config) -> DbConn {
    let database_url = format!(
        "mysql://{username}:{password}@{host}:{port}/{database_name}?{option}",
        username = config.db_username,
        password = config.db_password,
        host = config.db_host,
        port = config.db_port,
        database_name = config.db_name,
        option = config.db_option
    );
    // 创建数据库连接池
    let db_conn = Database::connect(&database_url)
        .await
        .expect("Failed to connect database.");
    db_conn
}

async fn init_cache(config: Config) -> Cache<String, Option<String>> {
    // 创建缓存
    let cache: Cache<String, Option<String>> = Cache::builder()
        .max_capacity(config.cache_max_cap)
        .time_to_live(Duration::from_secs(config.cache_live_time))
        .build();
    cache
}

async fn start(config: Config) -> std::io::Result<()> {
    let db_conn = init_db_conn(config.clone()).await;
    // 初始化数据库表结构
    init_db(db_conn.clone()).await;

    let cache = init_cache(config.clone()).await;

    // tera templates
    let templates = Tera::new("./templates/**/*").unwrap();

    // 共享数据
    let shared_state = AppState {
        config: config.clone(),
        templates,
        db_conn,
        cache,
    };

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "./static"))
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add((http::header::CONTENT_TYPE, "text/plain; charset=utf-8")),
            )
            .app_data(web::Data::new(shared_state.clone()))
            .default_service(web::route().to(not_found))
            .service(home)
            .service(generate_short_link)
            .service(redirect)
    })
    .bind(format!("0.0.0.0:{port}", port = config.port))?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let config = init_config();
    debug!("Config: {:?}", config);

    start(config).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    // #[actix_web::test]
    async fn test_index_ok() {
        let config = init_config();
        let db_conn = init_db_conn(config.clone()).await;
        let cache = init_cache(config.clone()).await;
        let templates = Tera::new("./templates/**/*").unwrap();

        let shared_state = AppState {
            config,
            db_conn,
            cache,
            templates,
        };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(shared_state))
                .service(home),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
