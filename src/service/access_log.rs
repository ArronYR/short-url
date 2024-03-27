use crate::models::access_log;
use log::error;
use sea_orm::{ActiveModelTrait, DatabaseBackend, DbConn, DbErr, FromQueryResult, Set, Statement};
use std::collections::HashMap;

pub struct AccessLogService;

impl AccessLogService {
    // 添加
    pub async fn add(
        db: &DbConn,
        data: access_log::Model,
    ) -> Result<access_log::ActiveModel, DbErr> {
        access_log::ActiveModel {
            short_id: Set(data.short_id.to_owned()),
            req_headers: Set(data.req_headers.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn batch_query_hits(db: &DbConn, short_ids: Vec<String>) -> HashMap<String, i64> {
        #[derive(Debug, FromQueryResult)]
        struct GroupByResult {
            short_id: String,
            total: i64,
        }

        let mut hits_map: HashMap<String, i64> = HashMap::new();
        if short_ids.is_empty() {
            return hits_map;
        }
        let id_string = short_ids
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<_>>()
            .join(",");

        let result = GroupByResult::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            format!("SELECT short_id, COUNT(1) AS total FROM `access_log` WHERE `short_id` IN ({}) GROUP BY `short_id`", id_string),
            [],
        ))
        .all(db)
        .await;

        if result.is_err() {
            error!("batch_query_hits error: {:?}", result.err());
            return hits_map;
        }

        for row in result.unwrap() {
            hits_map.insert(row.short_id, row.total);
        }
        hits_map
    }
}
