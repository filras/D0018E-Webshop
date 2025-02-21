use std::sync::OnceLock;

use tower_cookies::Key;

pub mod middleware;
pub mod routes;
pub mod ctx;

pub const COOKIE_NAME: &str = "AUTH_TOKEN";

pub static KEY: OnceLock<Key> = OnceLock::new();
