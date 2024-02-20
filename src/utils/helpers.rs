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
