use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use diesel::{
    self, dsl::delete, prelude::*, AsChangeset
};
use serde::Deserialize;

use crate::{
    auth::ctx::Ctx, db::{connect_to_db, models::User}, schema::users::{self as Users, dsl::users, *}
};

pub fn routes() -> Router {
    Router::new()
        .route("/users", 
            get(handle_get)
            .put(handle_put)
            .delete(handle_delete))
}


fn default_page() -> usize {
    1
}
fn default_per_page() -> usize {
    10
}
#[derive(Debug, Deserialize)]
struct GetUserQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_per_page")]
    per_page: usize,

    search: Option<String>,
}

// Query for users, uses search if present, otherwise all. Paginated
async fn handle_get(query: Query<GetUserQuery>) -> impl IntoResponse {
    let query: GetUserQuery = query.0;
    let conn = &mut connect_to_db();
    
    // Make different queries depending on if we're searching for username
    let query_results = match query.search {
        // Include only results filtered with search_string on username
        Some(search_string) => {
            users
                .select(User::as_select())
                .filter(
                    username.like(format!("%{}%",search_string))
                )
                .offset(((query.page - 1) * query.per_page) as i64)
                .limit(query.per_page as i64)
                .load::<User>(conn)
        },
        // Include all paginated results
        None => {
            users
                .select(User::as_select())
                .offset(((query.page - 1) * query.per_page) as i64)
                .limit(query.per_page as i64)
                .load::<User>(conn)
        }
    };

    // Make results into response
    match query_results {
        Ok(results) => (StatusCode::OK, Json(results)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

// Select user, used for PUT and DELETE
#[derive(Deserialize)]
struct UserQuery {
    id: i32,
}
// Having Options here means we will automatically ignore any fields not included in the query instead of writing these as null
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = Users)]
struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    firstname: Option<String>,
    surname: Option<String>,
    address: Option<String>,
    zipcode: Option<String>,
    co: Option<String>,
    country: Option<String>,
}

// Admins are allowed to edit any user's data except id and password
async fn handle_put(user: Query<UserQuery>, data: Json<UpdateUser>) -> impl IntoResponse {
    let rcv_user: UpdateUser = data.0;
    let user_id = user.0.id;
    
    
    let conn = &mut connect_to_db();
    return match diesel::update(users)
    .filter(id.eq(user_id))
    .set(rcv_user)
    .execute(conn)
    {
        Ok(_) => (StatusCode::OK, format!("User {} updated", user_id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
}


// Handles DELETE to delete and log out the current user
async fn handle_delete(ctx: Result<Ctx, String>, user: Query<UserQuery>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let user_id = user.0.id;

    // Cannot delete own user here (to prevent accidents)
    if ctx.unwrap().user_id() == user_id {
        return (StatusCode::BAD_REQUEST, "Cannot delete your own user here, use DELETE /api/account instead")
    }

    // Try to delete the user
    let result = delete(users.filter(id.eq(user_id))).execute(conn);
    
    // Return result
    match result {
        Ok(users_deleted) => match users_deleted > 0 {
            true => (StatusCode::OK, "User deleted"),
            false => (StatusCode::BAD_REQUEST, "No user found"),
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Account deletion failed"),
    }
}
