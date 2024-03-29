use crate::models::link;
use crate::models::link::{LinkStatusEnum, SearchParams};
use log::error;
use num_traits::ToPrimitive;
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set, UpdateResult,
};

pub struct LinkService;

impl LinkService {
    // 根据原始链接获取记录
    pub async fn find_by_original_url(db: &DbConn, url: String) -> Option<link::Model> {
        let record = link::Entity::find()
            .filter(link::Column::OriginalUrl.eq(url.clone()))
            .one(db)
            .await;

        record.unwrap_or_else(|e| {
            error!("[link service] find_by_original_url: {} error {}", url, e);
            None
        })
    }

    // 检查 short id 是否已被使用
    pub async fn check_short_id_used(db: &DbConn, short_id: String) -> bool {
        let record = link::Entity::find()
            .filter(link::Column::ShortId.eq(short_id.clone()))
            .one(db)
            .await;

        match record {
            Ok(row) => row.is_some(),
            Err(e) => {
                error!(
                    "[link service] check_short_id_used: {} error: {}",
                    short_id, e
                );
                false
            }
        }
    }

    // 根据 short id 获取记录
    pub async fn find_by_short_id(db: &DbConn, short_id: String) -> Option<link::Model> {
        let record = link::Entity::find()
            .filter(link::Column::ShortId.eq(short_id.clone()))
            .one(db)
            .await;

        record.unwrap_or_else(|e| {
            error!("[link service] find_by_short_id: {} error {}", short_id, e);
            None
        })
    }

    // 创建记录
    pub async fn create(db: &DbConn, data: link::Model) -> Result<link::ActiveModel, DbErr> {
        link::ActiveModel {
            short_id: Set(data.short_id.to_owned()),
            original_url: Set(data.original_url.to_owned()),
            expired_ts: Set(data.expired_ts.to_owned()),
            status: Set(data.status.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    // 搜索
    pub async fn search(
        db: &DbConn,
        params: SearchParams,
    ) -> Result<(Vec<link::Model>, u64), DbErr> {
        let mut selector = link::Entity::find();
        if let Some(keyword) = params.keyword {
            if !keyword.is_empty() {
                let keyword = format!("%{}%", keyword.clone());
                selector = selector.filter(
                    link::Column::ShortId
                        .like(keyword.clone())
                        .or(link::Column::OriginalUrl.like(keyword.clone())),
                );
            }
        }
        let paginator = selector
            .order_by_desc(link::Column::Id)
            .paginate(db, params.size.unwrap_or(30));
        let pages = paginator.num_pages().await?;

        // 获取分页结果
        paginator
            .fetch_page(params.page.unwrap_or(1) - 1)
            .await
            .map(|p| (p, pages))
    }

    pub async fn update_status(
        db: &DbConn,
        targets: &Vec<String>,
        status: &LinkStatusEnum,
    ) -> Result<UpdateResult, DbErr> {
        let result = link::Entity::update_many()
            .col_expr(link::Column::Status, Expr::value(status.to_i16()))
            .filter(link::Column::ShortId.is_in(targets.to_owned()))
            .exec(db)
            .await?;
        Ok(result)
    }

    pub async fn update_expired(
        db: &DbConn,
        targets: &Vec<String>,
        expired_ts: &i64,
    ) -> Result<UpdateResult, DbErr> {
        let result = link::Entity::update_many()
            .col_expr(link::Column::ExpiredTs, Expr::value(expired_ts.to_owned()))
            .filter(link::Column::ShortId.is_in(targets.to_owned()))
            .exec(db)
            .await?;
        Ok(result)
    }
}
