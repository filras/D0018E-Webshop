use axum::{
    extract::{Json, Query}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get}, Router
};
use diesel::{
    dsl::{self, insert_into, update}, prelude::*, result::Error
};
use serde::Serialize;
use tsync::tsync;

use crate::{
    auth::{ctx::Ctx, middleware::require_auth},
    db::{
        connect_to_db,
        models::{IdQuery, NewReview, PaginatedIdQuery, Review},
    },
    schema::{items, reviews, users},
};

pub fn routes() -> Router {
    Router::new()
        .route("/reviews", 
            get(get_reviews))
        .route("/reviews",
            delete(delete_review).post(create_review)
            .route_layer(middleware::from_fn(require_auth)))
}


#[derive(Serialize, Queryable)]
#[tsync]
struct ItemReview {
    pub user_id: i32,
    pub firstname: String,
    pub surname: String,
    pub comment: Option<String>,
    pub rating: i32,
}

// Perform a paginated GET for reviews on a certain item
async fn get_reviews(query: Query<PaginatedIdQuery>) -> impl IntoResponse {
    let query = query.0;
    let conn = &mut connect_to_db();
    
    // Make paginated query
    let query_results = reviews::table
        .filter(
            reviews::item_id.eq(query.id)
        )
        .offset(((query.page - 1) * query.per_page) as i64)
        .limit(query.per_page as i64)
        .inner_join(users::table)
        .select((
            users::id,
            users::firstname,
            users::surname,
            reviews::comment,
            reviews::rating,
        ))
        .load::<ItemReview>(conn);

    // Match results into response
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
    let result = dsl::delete(reviews::table
        .filter(reviews::user_id.eq(user.user_id()))
        .filter(reviews::item_id.eq(query.id))
    ).execute(conn);
    
    // Match result into response
    match result {
        Ok(_) => (StatusCode::OK, "Review deleted".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

// Handles POST to create the user's review for the specific item
async fn create_review(ctx: Result<Ctx, String>, query: Query<IdQuery>, data: Json<NewReview>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let query = query.0;
    let review = data.0;
    let conn = &mut connect_to_db();

    let values = (
        reviews::user_id.eq(user.user_id()),
        reviews::item_id.eq(query.id),
        reviews::rating.eq(review.rating),
        reviews::comment.eq(review.comment),
    );

    // Try to create the review
    let insert_result = insert_into(reviews::table)
        .values(values)
        .execute(conn);
    if insert_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Review creation failed")
    }

    // Update item rating
    let new_average_rating = calculate_average_rating(query.id, conn);
    if new_average_rating.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Rating calculation failed")
    }
    let item_update = update(items::table.find(query.id))
        .set(items::average_rating.eq(new_average_rating.unwrap()))
        .execute(conn);
    if item_update.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Rating update failed")
    }
    
    // Review creation succeeded
    (StatusCode::OK, "Review created")
}

// Returns the average rating of the item with item id `for_item_id`
fn calculate_average_rating(for_item_id: i32, conn: &mut MysqlConnection) -> Result<f32, Error> {
    // Get all items
    let result = reviews::table
        .select(Review::as_select())
        .filter(
            reviews::item_id.eq(for_item_id)
        )
        .load::<Review>(conn);

    // If result was Ok, calculate average rating, else pass the error
    match result {
        Ok(reviews_vec) => {
            let (mut amount, mut total_rating) = (0, 0);
            for rev in reviews_vec {
                total_rating += rev.rating;
                amount += 1;
            };

            // Return average
            Ok(total_rating as f32 / amount as f32)
        },
        Err(e) => Err(e),
    }
}
