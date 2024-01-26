use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "link")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub short_id: String,
    pub original_url: String,
    pub create_time: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// ======================== http request params ========================
#[derive(Debug, Deserialize)]
pub struct GenerateReq {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub id: Option<String>,
    pub url: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}
