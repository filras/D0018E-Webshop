use std::sync::OnceLock;

use tower_cookies::{Cookie, Cookies, Key};

pub mod middleware;
pub mod routes;

pub const AUTH_TOKEN: &str = "auth-token";

pub const COOKIE_NAME: &str = "AUTH_TOKEN";

pub static KEY: OnceLock<Key> = OnceLock::new();

pub async fn hander(cookies: Cookies) -> String {
  let key = KEY.get();
  let private_cookies = cookies.private(key.unwrap());

  let visited = private_cookies
      .get(COOKIE_NAME)
      .and_then(|c| c.value().parse().ok())
      .unwrap_or(0);
  if visited > 10 {
      cookies.remove(Cookie::new(COOKIE_NAME, ""));
      "Counter has been reset".into()
  } else {
      private_cookies.add(Cookie::new(COOKIE_NAME, (visited + 1).to_string()));
      format!("You've been here {} times before", visited)
  }
}
