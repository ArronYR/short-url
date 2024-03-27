use chrono::NaiveDateTime;
use num_derive::{FromPrimitive, ToPrimitive};
use sea_orm::entity::prelude::*;
use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "link")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub short_id: String,
    pub original_url: String,
    pub expired_ts: i64,
    pub status: i16,
    pub create_time: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// ======================== http request params ========================
#[derive(Debug, Deserialize)]
pub struct GenerateReq {
    pub urls: Vec<String>,
    #[serde(rename = "expiredTs")]
    pub expired_ts: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub keyword: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ChangeStatusReq {
    pub targets: Vec<String>,
    pub status: LinkStatusEnum,
}

#[derive(Debug, Deserialize)]
pub struct ChangeExpiredReq {
    pub targets: Vec<String>,
    pub expired: i64,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, ToPrimitive, FromPrimitive)]
#[repr(u16)]
pub enum LinkStatusEnum {
    Normal = 0,
    Disabled = 1,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchRecordItem {
    pub id: u64,
    #[serde(rename = "shortId")]
    pub short_id: String,
    #[serde(rename = "originalUrl")]
    pub original_url: String,
    #[serde(rename = "expiredTs")]
    pub expired_ts: i64,
    pub status: i16,
    #[serde(rename = "createTime")]
    pub create_time: NaiveDateTime,
    pub hits: i64,
}
