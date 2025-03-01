use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use diesel::{
    self,
    dsl::{delete, insert_into},
    prelude::*,
    AsChangeset,
};
use serde::Deserialize;

use crate::{
    auth::{self, ctx::Ctx, session},
    db::{connect_to_db, models::User},
    schema::users::{self as Users, dsl::users, *},
};

pub fn routes() -> Router {
    Router::new()
        .route("/users", 
            get(handle_get))
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
async fn handle_get(pagination: Query<GetUserQuery>) -> impl IntoResponse {
    let pagination: GetUserQuery = pagination.0;
    let conn = &mut connect_to_db();

    return match users
        .offset(((pagination.page - 1) * pagination.per_page) as i64)
        .limit(pagination.per_page as i64)
        .select(User::as_select())
        .load::<User>(conn) {
            Ok(results) => (StatusCode::OK, Json(results)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
}

