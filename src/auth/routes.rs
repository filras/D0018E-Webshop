use axum::{
    handler::Handler, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Json, Router
};
use diesel::{dsl::insert_into, query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl, SelectableHelper};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::{
    auth::{self, ctx::Ctx},
    db::{
        connect_to_db,
        models::User,
        schema::users::{dsl::users, *},
    },
};

use super::{COOKIE_NAME, KEY};

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub password: String,
    pub firstname: String,
    pub surname: String,
    pub email: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(handle_login))
        .route("/register", post(handle_register))
        .route("/logout", get(handle_logout
            .layer(middleware::from_fn(auth::middleware::mw_require_auth)))) // Can only log out if already logged in
}

async fn handle_login(
    cookies: Cookies,
    ctx: Result<Ctx, String>,
    login: Json<Login>,
) -> impl IntoResponse {
    // If already logged in, return an error
    if ctx.is_ok() {
        return (StatusCode::BAD_REQUEST, "Already logged in").into_response()
    }

    // Search for users in the DB with the requested username
    let conn = &mut connect_to_db();
    let result = users.filter(username.eq(login.username.clone()))
        .select(User::as_select())
        .load::<User>(conn);

    // Handle error conditions
    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", result.err().unwrap())).into_response()
    }
    let retrieved_users = result.unwrap();
    if retrieved_users.len() > 1 {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error: found duplicate users").into_response()
    }
    // Handle user not found
    else if retrieved_users.len() == 0 {
        return (StatusCode::BAD_REQUEST, format!("Could not find user '{}'", login.username)).into_response()
    }

    // We have verified that there is only one retrieved user, so unwrap it
    let user = retrieved_users.first().unwrap();

    // Verify the password hash with bcrypt
    let verification_result = bcrypt::verify(login.password.to_string(), user.password_hash.as_str());

    if verification_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Bcrypt error: {}", verification_result.err().unwrap())).into_response()
    } else if !verification_result.unwrap() {
        return (StatusCode::UNAUTHORIZED, "Incorrect password").into_response();
    }

    // Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());

    if private_cookies.get(COOKIE_NAME).is_none() {
        let mut cookie = Cookie::new(COOKIE_NAME, user.id.to_string());
        cookie.set_http_only(true);
        cookie.set_path("/");
        private_cookies.add(cookie);
    }

    format!("Logged in as {}", user.username).into_response()
}

async fn handle_logout(
    cookies: Cookies,
) -> impl IntoResponse {
    // Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());
    // Remove cookie
    private_cookies.remove(Cookie::build(COOKIE_NAME).path("/").into());

    "Logged out".into_response()
}

async fn handle_register(data: Json<NewUser>) -> impl IntoResponse {
    let rcv_user: NewUser = data.0;

    // Create password hash
    let hash = bcrypt::hash(rcv_user.password, 12);
    if hash.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to hash password").into_response()
    }

    let conn = &mut connect_to_db();
    let values = (
        username.eq(rcv_user.email.clone()),
        password_hash.eq(hash.unwrap()),
        firstname.eq(rcv_user.firstname),
        surname.eq(rcv_user.surname),
        email.eq(rcv_user.email),
        role.eq("customer"),
    );

    return match insert_into(users)
        .values(values)
        .execute(conn) {
        Ok(_) => (StatusCode::OK, "User recieved").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
