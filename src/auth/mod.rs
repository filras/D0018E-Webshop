use std::sync::OnceLock;

use tower_cookies::Key;

pub mod middleware;
pub mod session;
pub mod ctx;

pub const COOKIE_NAME: &str = "AUTH_TOKEN";
pub const COOKIE_EXPIRATION_TIME_SECONDS: i64 = 60*60; // 1 hour

pub static KEY: OnceLock<Key> = OnceLock::new();
