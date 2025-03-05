use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use diesel::{dsl::delete, prelude::*};

use crate::{
    auth::ctx::Ctx,
    db::{
        connect_to_db,
        models::{IdQuery, PaginatedIdQuery, Review},
    }, schema::reviews::{dsl::reviews, item_id, user_id}
};

pub fn routes() -> Router {
    Router::new()
        .route("/reviews", 
            get(get_reviews)
            .delete(delete_review))
}

// Perform a paginated GET for reviews on a certain item
async fn get_reviews(query: Query<PaginatedIdQuery>) -> impl IntoResponse {
    let query = query.0;
    let conn = &mut connect_to_db();
    
    // Make paginated query
    let query_results = reviews
        .select(Review::as_select())
        .filter(
            item_id.eq(query.id)
        )
        .offset(((query.page - 1) * query.per_page) as i64)
        .limit(query.per_page as i64)
        .load::<Review>(conn);

    // Make results into response
    match query_results {
        Ok(results) => (StatusCode::OK, Json(results)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

// Handles DELETE to delete the user's review for the specific item
async fn delete_review(ctx: Result<Ctx, String>, query: Query<IdQuery>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let query = query.0;
    let conn = &mut connect_to_db();

    // Try to delete the review
    let result = delete(reviews
        .filter(user_id.eq(user.user_id()))
        .filter(item_id.eq(query.id))
    ).execute(conn);
    
    if result.is_ok() {
        return (StatusCode::OK, "Review deleted")
    }

    // Review deletion failed
    (StatusCode::INTERNAL_SERVER_ERROR, "Review deletion failed")
}
