use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use diesel::prelude::*;

use crate::{
    db::{
        connect_to_db,
        models::{IdQuery, Item, PaginatedSearchQuery},
    }, schema::items::{dsl::items, title}
};

pub fn routes() -> Router {
    Router::new()
        .route("/items", get(get_items))
        .route("/item", get(get_item_by_id))
}

// Perform a paginated GET for items, with optional search string
async fn get_items(query: Query<PaginatedSearchQuery>) -> impl IntoResponse {
    let query: PaginatedSearchQuery = query.0;
    let conn = &mut connect_to_db();
    
    // Make different queries depending on if we're searching for username
    let query_results = match query.search {
        // Include only results filtered with search_string on username
        Some(search_string) => {
            items
                .select(Item::as_select())
                .filter(
                    title.like(format!("%{}%",search_string))
                )
                .offset(((query.page - 1) * query.per_page) as i64)
                .limit(query.per_page as i64)
                .load::<Item>(conn)
        },
        // Include all paginated results
        None => {
            items
                .select(Item::as_select())
                .offset(((query.page - 1) * query.per_page) as i64)
                .limit(query.per_page as i64)
                .load::<Item>(conn)
        }
    };

    // Make results into response
    match query_results {
        Ok(results) => (StatusCode::OK, Json(results)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

// Perform a GET for a specific item
async fn get_item_by_id(query: Query<IdQuery>) -> impl IntoResponse {
    let query = query.0;
    let conn = &mut connect_to_db();
    
    // Make different queries depending on if we're searching for username
    let query_results = items
                .select(Item::as_select())
                .find(query.id)
                .first::<Item>(conn);

    // Make results into response
    match query_results {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
