use axum::{routing::{get, post}, Json, Router};
use axum_session_auth::*;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::{self, Ctx};

use super::{AUTH_TOKEN, COOKIE_NAME, KEY};

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
    // auth: AuthSession<User, i64, SessionNullPool, NullPool>,
    // ctx: Ctx,
    cookies: Cookies,
    login: Json<Login>,
) -> String {
    let mut found_user = false;
    let mut user: Option<&TestUser> = None;

    // TEMP: Search through test users to find the one with the right user/pass combo
    for test_user in USERS {
        if login.username == test_user.username && login.password == test_user.password {
            // auth.login_user(test_user.id);
            println!("yay we found something");
            // println!("{}", ctx.user_id());
            found_user = true;
            user = Some(test_user);
        }
    }

    if !found_user { return "bad skibidi".to_owned() };

    // Create cookie
	let key = KEY.get();
    let private_cookies = cookies.private(key.unwrap());

    if private_cookies.get(COOKIE_NAME).is_none() {
        private_cookies.add(Cookie::new(COOKIE_NAME, user.unwrap().id.to_string()));
    }


    "Logged in".to_owned()
}

async fn handle_logout(
    // auth: AuthSession<User, i64, SessionNullPool, NullPool>
) -> String {
    // auth.logout_user();
    "Logged out".to_owned()
}
