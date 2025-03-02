use axum::{
    extract::Json,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use diesel::{self, dsl::{delete, insert_into}, prelude::*};
use tower_cookies::Cookies;

use crate::{
    auth::{self, session, ctx::Ctx},
    db::{connect_to_db, models::{User, UpdateUser, NewUser}},
    schema::users::{dsl::users, *},
};

pub fn routes() -> Router {
    Router::new()
        .route("/account", post(handle_post))
        .route("/account",
            get(handle_get)
            .put(handle_put)
            .delete(handle_delete)
                .layer(middleware::from_fn(auth::middleware::require_auth)), // Can only get/edit/delete account if logged in
    )
}

// Handles POST to register a new user
async fn handle_post(ctx: Result<Ctx, String>, cookies: Cookies, data: Json<NewUser>) -> impl IntoResponse {
    // If already logged in, return an error
    if ctx.is_ok() {
        return (StatusCode::BAD_REQUEST, "Already logged in").into_response()
    }

    let rcv_user: NewUser = data.0;
    let conn = &mut connect_to_db();
    
    // Check for duplicate user
    let user_exists_result = users
        .filter(username.eq(rcv_user.email.clone()))
        .select(User::as_select())
        .first::<User>(conn);
    if user_exists_result.is_ok() {
        return (StatusCode::BAD_REQUEST, "Username taken").into_response()
    }

    // Create password hash
    let hash = bcrypt::hash(rcv_user.password, 12);
    if hash.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to hash password").into_response()
    }

    // Special case: if the username is given as "admin", set their role to admin
    // This is to allow the first user to become admin and create other admins
    let new_user_role = match rcv_user.email == "admin" {
        true => "admin".to_string(),
        false => "customer".to_string(),
    };

    let values = (
        username.eq(rcv_user.email.clone()),
        password_hash.eq(hash.unwrap()),
        firstname.eq(rcv_user.firstname),
        surname.eq(rcv_user.surname),
        email.eq(rcv_user.email.clone()),
        role.eq(new_user_role),
    );

    // Insert new user into DB
    let insert_result = insert_into(users)
        .values(values)
        .execute(conn);
    if insert_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error while creating the account").into_response()
    }
    
    // Query the DB for the new user to retrieve the user's id and create a session for them
    let read_result = users
        .filter(username.eq(rcv_user.email.clone()))
        .select(User::as_select())
        .first::<User>(conn);
    if read_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to fetch user id, try logging in").into_response()
    }

    // Create user session and return success
    let user = read_result.unwrap();
    session::create_user_session(cookies, user.id);
    return (StatusCode::OK, format!("Created account {}", user.username)).into_response()
}

async fn handle_put(ctx: Result<Ctx, String>, data: Json<UpdateUser>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let rcv_user: UpdateUser = data.0;
    let conn = &mut connect_to_db();

    return match diesel::update(users)
        .filter(id.eq(user.user_id()))
        .set(rcv_user)
        .execute(conn)
    {
        Ok(_) => (StatusCode::OK, "User updated").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
}

// Handles DELETE to delete and log out the current user
async fn handle_delete(cookies: Cookies, ctx: Result<Ctx, String>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();

    // Try to delete the user
    let result = delete(users.filter(id.eq(user.user_id()))).execute(conn);
    
    if result.is_ok() {
        // Remove cookie and return success
        session::remove_user_session(cookies);
        return (StatusCode::OK, "User deleted")
    }

    // Account deletion failed
    (StatusCode::INTERNAL_SERVER_ERROR, "Account deletion failed")
}

async fn handle_get(ctx: Result<Ctx, String>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();
    return match users
        .filter(id.eq(user.user_id()))
        .select(User::as_select())
        .first::<User>(conn)
    {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
}
