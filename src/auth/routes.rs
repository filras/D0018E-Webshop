use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;

use super::{COOKIE_NAME, KEY};

// TEMP
struct TestUser {
    pub username: &'static str,
    pub password: &'static str,
    pub id: i64,
}

const USERS: &'static [TestUser] = &[
    TestUser {
        id: 123,
        username: "Olle",
        password: "sus",
    },
    TestUser {
        id: 456,
        username: "admin",
        password: "admin",
    },
    TestUser {
        id: 789,
        username: "Viggo",
        password: "test",
    },
];

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/auth/login", post(handle_login))
        .route("/auth/logout", get(handle_logout))
}

async fn handle_login(
    cookies: Cookies,
    login: Json<Login>,
) -> impl IntoResponse {
    let mut found_user = false;
    let mut user: Option<&TestUser> = None;

    // TEMP: Search through test users to find the one with the right user/pass combo
    for test_user in USERS {
        if login.username == test_user.username && login.password == test_user.password {
            found_user = true;
            user = Some(test_user);
        }
    }

    if !found_user { return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response() };

    // Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());

    if private_cookies.get(COOKIE_NAME).is_none() {
        let mut cookie = Cookie::new(COOKIE_NAME, user.unwrap().id.to_string());
        cookie.set_http_only(true);
        cookie.set_path("/");
        private_cookies.add(cookie);
    }


    "Logged in".into_response()
}

async fn handle_logout(
    _ctx: Ctx, // Require context, cannot log out if not logged in
    cookies: Cookies,
) -> impl IntoResponse {
    // Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());
    // Remove cookie
    private_cookies.remove(Cookie::build(COOKIE_NAME).path("/").into());

    "Logged out".into_response()
}
