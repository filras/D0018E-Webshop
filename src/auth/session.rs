use axum::{
    handler::Handler, http::StatusCode, middleware, response::IntoResponse, routing::{get, post, put}, Json, Router
};
use diesel::{query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl, SelectableHelper};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::{
    auth::{self, ctx::Ctx},
    schema::users::{dsl::users, *},
    db::{
        connect_to_db,
        models::User,
    },
};

use super::{COOKIE_NAME, KEY};

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/me", get(handle_my_user))
        .route("/login", post(handle_login))
        .route("/logout", get(handle_logout)
            .layer(middleware::from_fn(auth::middleware::require_auth))) // Can only log out if already logged in
        .route("/password", put(handle_change_password)
            .layer(middleware::from_fn(auth::middleware::require_auth))) // May only change password if logged in
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
        .first::<User>(conn);

    // Handle error conditions
    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", result.err().unwrap())).into_response()
    }
    
    // Verify the password hash with bcrypt
    let user = result.unwrap();
    let verification_result = bcrypt::verify(login.password.to_string(), user.password_hash.as_str());

    if verification_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Bcrypt error: {}", verification_result.err().unwrap())).into_response()
    } else if !verification_result.unwrap() {
        return (StatusCode::UNAUTHORIZED, "Incorrect password").into_response();
    }

    create_user_session(cookies, user.id);

    format!("Logged in as {}", user.username).into_response()
}

// Handles the /logout path that logs out the current user if they are signed in
async fn handle_logout(
    cookies: Cookies,
    ctx: Result<Ctx, String>,
) -> impl IntoResponse {
    remove_user_session(cookies);

    format!("Logged out from {}", ctx.unwrap().username())
}

// Handles the /me path, to quickly know if you're logged in, and if so with which account
async fn handle_my_user(ctx: Result<Ctx, String>) -> impl IntoResponse {
    match ctx {
        Ok(user) => format!("Currently logged in as {}. Is admin? {}", user.username(), user.is_admin()),
        Err(_) => "Not logged in".to_string(),
    }
}

#[derive(Deserialize)]
pub struct ChangePassword {
    old_password: String,
    new_password: String,
}

async fn handle_change_password(ctx: Result<Ctx, String>, password: Json<ChangePassword>) -> impl IntoResponse {
    let user_id = ctx.unwrap().user_id();
    // Get the user password from the database
    let conn = &mut connect_to_db();
    let result = users.filter(id.eq(user_id))
        .select(User::as_select())
        .first::<User>(conn);

    // Handle error conditions
    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", result.err().unwrap())).into_response()
    }

    // Verify the password hash with bcrypt
    let user = result.unwrap();
    let verification_result = bcrypt::verify(password.old_password.to_string(), user.password_hash.as_str());

    if verification_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Bcrypt error: {}", verification_result.err().unwrap())).into_response()
    } else if !verification_result.unwrap() {
        return (StatusCode::BAD_REQUEST, "Incorrect old password").into_response()
    }

    // Generate a new hash
    let new_hash = bcrypt::hash(password.new_password.to_string(), 12);
    if new_hash.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to hash password").into_response()
    }
    // Set new password hash in DB
    return match diesel::update(users)
        .filter(id.eq(user_id))
        .set(password_hash.eq(new_hash.unwrap()))
        .execute(conn)
    {
        Ok(_) => (StatusCode::OK, "Changed password").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
}

// Create cookie for user with the given user id
pub fn create_user_session(cookies: Cookies, user_id: i32) {
    // Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());

    // Add cookie if it doesn't already exist
    if private_cookies.get(COOKIE_NAME).is_none() {
        let mut cookie = Cookie::new(COOKIE_NAME, user_id.to_string());
        cookie.set_http_only(true);
        cookie.set_path("/");
        private_cookies.add(cookie);
    }
}

pub fn remove_user_session(cookies: Cookies) {
    // Create private cookie jar from global static KEY to validate cookies
    let key = KEY.get();
    let private_cookies = cookies.private(key.unwrap());
    // Remove cookie
    private_cookies.remove(Cookie::build(COOKIE_NAME).path("/").into());
}
