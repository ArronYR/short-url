pub mod middleware;
pub mod models;
pub mod service;
pub mod utils;

use moka::future::Cache;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: String,
    pub origin: String,
    pub db_host: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub db_option: String,
    pub token: String,
    pub cache_max_cap: u64,
    pub cache_live_time: u64,
    pub api_secret: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub cache: Cache<String, Option<String>>,
    pub templates: tera::Tera,
    pub db_conn: DatabaseConnection,
}
