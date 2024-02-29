use actix_web::http::header::HeaderValue;
use chrono::{LocalResult, TimeZone, Utc};
use rand::distributions::Alphanumeric;
use rand::Rng;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use url::Url;

// 生成短链接ID
pub fn generate_short_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}

// 检查是否是Url
pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

pub fn hash_256_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn hash_1_hex(data: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn md5_hex(data: &str) -> String {
    let result = md5::compute(data);
    hex::encode(result.0)
}

pub fn validate_header_token(header_value: Option<&HeaderValue>, token: &str) -> bool {
    let value = match header_value {
        Some(header_value) => match header_value.to_str() {
            Ok(value) => value.to_string(),
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    };
    value == token
}

pub fn is_reasonable_timestamp(value: i64) -> bool {
    if value == 0 {
        return true;
    }
    let dt = Utc.timestamp_opt(value, 0);
    match dt {
        LocalResult::None => false,
        LocalResult::Single(dt) => dt.timestamp() >= Utc::now().timestamp(),
        LocalResult::Ambiguous(dt, _) => dt.timestamp() >= Utc::now().timestamp(),
    }
}
