use actix_cors::Cors;
use actix_web::rt::spawn;
use actix_web::{
    get, http, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use chrono::{NaiveDateTime, Utc};
use log::{debug, error, info, warn};
use moka::future::Cache;
use num_traits::ToPrimitive;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbConn};
use serde_json::json;
use short_url::middleware::validator::ApiValidateMiddleware;
use short_url::models::link::SearchRecordItem;
use short_url::models::{
    access_log::Model as AccessLogModel,
    link::{
        ChangeExpiredReq, ChangeStatusReq, GenerateReq, LinkStatusEnum, Model as LinkModel,
        SearchParams,
    },
};
use short_url::service::access_log::AccessLogService;
use short_url::service::link::LinkService;
use short_url::utils::helpers::{
    generate_short_id, is_reasonable_timestamp, is_valid_url, md5_hex, validate_header_token,
};
use short_url::{AppState, Config};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use tera::Tera;

const SCHEMA: &str = include_str!("../db.sql");
const DB_OPTION: &str = "characterEncoding=utf-8&characterSetResults=utf-8&autoReconnect=true&failOverReadOnly=false&serverTimezone=GMT%2B8";
const TOKEN: &str = "53ROYinHId9qke";
const API_SECRET: &str = "1FIsiEpxQo5l7H";
const HEADER_TOKEN_KEY: &str = "Token";

// 初始化配置信息
fn init_config() -> Config {
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let origin = env::var("ORIGIN").unwrap_or_else(|_| "https://127.0.0.1".to_string());
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
    let api_secret = env::var("API_SECRET").unwrap_or_else(|_| API_SECRET.to_string());
    let access_log = match env::var("ACCESS_LOG") {
        Ok(value) => value.parse::<bool>().unwrap_or(true),
        Err(_) => true,
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
        api_secret,
        access_log,
    }
}

async fn build_error_page(
    request: HttpRequest,
    state: web::Data<AppState>,
    template_name: &str,
) -> String {
    let template = &state.templates;

    let mut ctx = tera::Context::new();
    ctx.insert("uri", request.uri().path());

    template
        .render(template_name, &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
        .unwrap_or_else(|_| "error".to_string())
}

// 404
async fn not_found(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(build_error_page(request, state, "error/404.html.tera").await)
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
    let config = &state.config;

    let short_id = short_id.as_str();
    // 写入日志
    if config.access_log {
        let short_id = short_id.to_string().clone();
        let db_conn = db_conn.clone();
        let headers = request.headers().clone();

        spawn(async move {
            let req_headers = headers
                .iter()
                .map(|(k, v)| format!("{}: {:?}", k, v))
                .collect::<Vec<String>>()
                .join("\n");
            let model = AccessLogModel {
                id: 0,
                short_id: short_id.clone(),
                req_headers,
                create_time: NaiveDateTime::default(),
            };
            if let Err(e) = AccessLogService::add(&db_conn, model).await {
                error!("add id: {} access log error: {:?}", short_id, e);
            };
        });
    }

    // 如果缓存存在
    if let Some(cached) = cache.get(short_id).await {
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
            .body(build_error_page(request, state, "error/404.html.tera").await);
    }
    let result = LinkService::find_by_short_id(&db_conn, short_id.to_string()).await;
    if let Some(row) = result {
        if row.original_url.is_empty() {
            error!("[redirect] short_id: {} original_url is empty", short_id);
            cache.insert(short_id.to_string(), None).await;
            HttpResponse::NotFound().finish()
        } else {
            info!(
                "[redirect] short_id: {} original_url: {} status: {}",
                short_id, row.original_url, row.status
            );
            // 已被禁用
            if row.status == LinkStatusEnum::Disabled.to_i16().unwrap() {
                cache.insert(short_id.to_string(), None).await;
                return HttpResponse::Ok()
                    .content_type("text/html")
                    .body(build_error_page(request, state, "disabled.html.tera").await);
            }
            // 已过期
            if row.expired_ts > 0 && row.expired_ts < Utc::now().timestamp_millis() {
                cache.insert(short_id.to_string(), None).await;
                return HttpResponse::Ok()
                    .content_type("text/html")
                    .body(build_error_page(request, state, "expired.html.tera").await);
            }
            let original_url: String = row.original_url;
            cache
                .insert(short_id.to_string(), Some(original_url.clone()))
                .await;
            HttpResponse::TemporaryRedirect()
                .append_header(("Location", original_url))
                .finish()
        }
    } else {
        error!("[redirect] short_id: {} is not found", short_id);
        cache.insert(short_id.to_string(), None).await;
        HttpResponse::Ok()
            .content_type("text/html")
            .body(build_error_page(request, state, "error/404.html.tera").await)
    }
}

#[get("/api/search")]
async fn search(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let db_conn = &state.db_conn;
    let config = &state.config;

    // get params
    let params = web::Query::<SearchParams>::from_query(req.query_string()).unwrap();
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(30);

    let params = SearchParams {
        keyword: params.keyword.clone(),
        page: Some(page),
        size: Some(size),
    };

    let (links, pages) = LinkService::search(&db_conn, params)
        .await
        .expect("Cannot find links in page");

    // 查找Hits数据（如果开启了ACCESS_LOG）
    let mut hits_map: HashMap<String, i64> = HashMap::new();
    if config.access_log {
        let ids: Vec<String> = links.iter().map(|r| r.short_id.clone()).collect();
        hits_map = AccessLogService::batch_query_hits(&db_conn, ids).await;
    }

    let mut records: Vec<SearchRecordItem> = Vec::new();
    for link in links {
        records.push(SearchRecordItem {
            id: link.id,
            short_id: link.short_id.clone(),
            original_url: link.original_url,
            expired_ts: link.expired_ts,
            status: link.status,
            create_time: link.create_time,
            hits: *hits_map.get(link.short_id.as_str()).unwrap_or(&0i64),
        })
    }

    let json = json!({
        "records": records,
        "pages": pages,
        "size": size
    });
    let resp = serde_json::to_string(&json).expect("Serialize error response failed");
    Ok(HttpResponse::Ok()
        .content_type("application/json;utf-8")
        .body(resp))
}

#[get("/{filename:.*}")]
async fn static_files(req: HttpRequest) -> impl Responder {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = format!("./web/{}", path.to_string_lossy());
    actix_files::NamedFile::open(path)
}

// 创建短链接的处理函数
#[post("/api/generate")]
async fn generate(
    request: HttpRequest,
    state: web::Data<AppState>,
    params: web::Json<GenerateReq>,
) -> impl Responder {
    let db_conn = &state.db_conn;
    let config = &state.config;

    if !validate_header_token(
        request.headers().get(HEADER_TOKEN_KEY),
        config.token.as_str(),
    ) {
        return HttpResponse::BadRequest().body("请提供正确的安全码");
    }

    let expired_ts = params.expired_ts.unwrap_or(0);
    let mut results = HashMap::new();
    for url in &params.urls {
        let url = url.as_str().trim();
        // 检查URL是否合法
        if !is_valid_url(url) {
            return HttpResponse::BadRequest().body("请提供正确的链接");
        }
        let short_id = generate_to_db(db_conn.clone(), url, expired_ts).await;
        let short_link = match short_id {
            Some(short_id) => {
                format!("{origin}/{id}", origin = config.origin, id = short_id)
            }
            None => "".to_string(),
        };
        results.insert(md5_hex(url), short_link);
    }
    let resp = serde_json::to_string(&results).expect("Serialize error response failed");
    HttpResponse::Ok()
        .content_type("application/json;utf-8")
        .body(resp)
}

async fn generate_to_db(db_conn: DatabaseConnection, url: &str, expired_ts: i64) -> Option<String> {
    // 如果源链接key已存在，直接返回
    if let Some(link) = LinkService::find_by_original_url(&db_conn, url.to_string()).await {
        return Some(link.short_id);
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
            return None;
        }
    }
    let model = LinkModel {
        id: 0,
        short_id: short_id.clone(),
        original_url: url.to_string(),
        expired_ts,
        status: 0,
        create_time: NaiveDateTime::default(),
    };
    let result = LinkService::create(&db_conn, model).await;
    return match result {
        Ok(_) => {
            info!("generate url: {} success: {}", url, short_id);
            Some(short_id)
        }
        Err(e) => {
            error!("generate url: {} err: {:?}", url, e);
            None
        }
    };
}

#[post("/api/status")]
async fn change_status(
    request: HttpRequest,
    state: web::Data<AppState>,
    params: web::Json<ChangeStatusReq>,
) -> impl Responder {
    let db_conn = &state.db_conn;
    let config = &state.config;
    let cache = &state.cache;

    if !validate_header_token(
        request.headers().get(HEADER_TOKEN_KEY),
        config.token.as_str(),
    ) {
        return HttpResponse::BadRequest().body("请提供正确的安全码");
    }

    let result = LinkService::update_status(&db_conn, &params.targets, &params.status).await;
    debug!("update status result: {:?}", result);
    match result {
        Ok(_) => {
            remove_cache(cache, &params.targets).await;
            HttpResponse::Ok()
                .content_type("application/json;utf-8")
                .body("{}")
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/api/expired")]
async fn change_expired(
    request: HttpRequest,
    state: web::Data<AppState>,
    params: web::Json<ChangeExpiredReq>,
) -> impl Responder {
    let db_conn = &state.db_conn;
    let config = &state.config;
    let cache = &state.cache;

    if !validate_header_token(
        request.headers().get(HEADER_TOKEN_KEY),
        config.token.as_str(),
    ) {
        return HttpResponse::BadRequest().body("请提供正确的安全码");
    }

    if !is_reasonable_timestamp(params.expired) {
        return HttpResponse::BadRequest().body("请提供不小于当前日期的过期时间");
    }

    let result = LinkService::update_expired(&db_conn, &params.targets, &params.expired).await;
    debug!("update expired result: {:?}", result);
    match result {
        Ok(_) => {
            remove_cache(cache, &params.targets).await;
            HttpResponse::Ok()
                .content_type("application/json;utf-8")
                .body("{}")
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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

async fn remove_cache(cache: &Cache<String, Option<String>>, keys: &Vec<String>) {
    for key in keys {
        cache.remove(key).await;
    }
}

async fn start(config: Config) -> std::io::Result<()> {
    let db_conn = init_db_conn(config.clone()).await;
    // 初始化数据库表结构
    init_db(db_conn.clone()).await;
    // 初始化缓存配置
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
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add((http::header::CONTENT_TYPE, "text/html; charset=utf-8")),
            )
            .wrap(ApiValidateMiddleware {
                secret: config.api_secret.clone(),
            })
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(shared_state.clone()))
            .default_service(web::route().to(not_found))
            .service(redirect)
            .service(search)
            .service(generate)
            .service(change_status)
            .service(change_expired)
            .service(actix_files::Files::new("/", "./web").index_file("index.html"))
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

    #[actix_web::test]
    #[ignore]
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
                .service(redirect),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_ts() {
        let ts = Utc::now().timestamp_millis();
        println!("now timestamp: {}", ts);
    }
}
