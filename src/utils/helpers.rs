use rand::distributions::Alphanumeric;
use rand::Rng;
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
