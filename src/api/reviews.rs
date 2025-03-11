use axum::{
    extract::{Json, Query}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get}, Router
};
use diesel::{
    dsl::{self, insert_into, update}, prelude::*, result::Error
};
use serde::{Deserialize, Serialize};
use tsync::tsync;

use crate::{
    auth::{ctx::Ctx, middleware::require_auth},
    db::{
        connect_to_db,
        models::{Comment, IdQuery, PaginatedIdQuery, Review},
    },
    schema::{comments, items, reviews, users},
};

pub fn routes() -> Router {
    Router::new()
        .route("/reviews", 
            get(get_reviews))
        .route("/reviews",
            delete(delete_review).post(create_review)
            .route_layer(middleware::from_fn(require_auth)))
        .route("/comments",
            delete(delete_comment).post(create_comment)
            .route_layer(middleware::from_fn(require_auth)))
}


#[derive(Serialize, Queryable, Debug)]
struct ItemReview {
    pub user_id: i32,
    pub review_id: i32,
    pub firstname: String,
    pub surname: String,
    pub comment: Option<String>,
    pub rating: i32,
}

#[derive(Serialize, Queryable, Debug)]
#[tsync]
struct ItemReviewWithComments {
    pub user_id: i32,
    pub review_id: i32,
    pub firstname: String,
    pub surname: String,
    pub comment: Option<String>,
    pub rating: i32,
    pub comments: Vec<ReviewComment>,
}

#[derive(Serialize, Queryable, Debug)]
#[tsync]
struct ReviewComment {
    pub id: i32,
    pub user_id: i32,
    pub review_id: i32,
    pub firstname: String,
    pub surname: String,
    pub comment: String,
    pub comment_id: Option<i32>,
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
            reviews::id,
            users::firstname,
            users::surname,
            reviews::comment,
            reviews::rating,
        ))
        .load::<ItemReview>(conn);
    if query_results.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, query_results.unwrap_err().to_string()).into_response()
    }

    // Get all comments from their respective reviews
    let reviews = query_results.unwrap();
    let mut reviews_with_comments: Vec<ItemReviewWithComments> = vec![];
    for review in reviews {
        let query_results = comments::table
            .filter(
                comments::review_id.eq(review.review_id)
            )
            .inner_join(reviews::table)
            .inner_join(users::table)
            .select((
                comments::id,
                users::id,
                reviews::id,
                users::firstname,
                users::surname,
                comments::comment,
                comments::comment_id,
            ))
            .load::<ReviewComment>(conn);
        if query_results.is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR, query_results.unwrap_err().to_string()).into_response()
        }
        let comments = query_results.unwrap();

        reviews_with_comments.push(ItemReviewWithComments {
            user_id: review.user_id,
            review_id: review.review_id,
            firstname: review.firstname,
            surname: review.surname,
            comment: review.comment,
            rating: review.rating,
            comments: comments,
        });
    }

    // Return reviews with comments added
    (StatusCode::OK, Json(reviews_with_comments)).into_response()
}

// Handles DELETE to delete the user's review for the specific item
async fn delete_review(ctx: Result<Ctx, String>, query: Query<IdQuery>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let query = query.0;
    let conn = &mut connect_to_db();

    // Delete all child comments
    let review_to_delete = reviews::table
        .select(Review::as_select())
        .filter(reviews::user_id.eq(user.user_id()))
        .filter(reviews::item_id.eq(query.id))
        .first::<Review>(conn);
    if review_to_delete.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, review_to_delete.unwrap_err().to_string())
    }
    // Run delete query for all comments belonging to this review
    let delete_children = dsl::delete(Comment::belonging_to(&review_to_delete.unwrap()))
        .execute(conn);
    if delete_children.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, delete_children.unwrap_err().to_string())
    }

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

#[derive(Deserialize)]
#[tsync]
pub struct NewReview {
    pub comment: Option<String>,
    pub rating: i32,
}

// Handles POST to create the user's review for the specific item
async fn create_review(ctx: Result<Ctx, String>, query: Query<IdQuery>, data: Json<NewReview>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let query = query.0;
    let review = data.0;
    let conn = &mut connect_to_db();

    // Enforce only one review per user (cheaper and easier to do this in backend than as a SQL constraint)
    let review_already_exists = reviews::table
        .select(Review::as_select())
        .filter(reviews::user_id.eq(user.user_id()))
        .filter(reviews::item_id.eq(query.id))
        .first::<Review>(conn);
    if review_already_exists.is_ok() {
        return (StatusCode::BAD_REQUEST, "You have already reviewed this item")
    }

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


#[derive(Deserialize)]
#[tsync]
pub struct NewComment {
    pub comment: String,
    pub parent_id: Option<i32>,
}

// Handles POST to create a response comment under a review, optionally linked to another comment as child
async fn create_comment(ctx: Result<Ctx, String>, query: Query<IdQuery>, data: Json<NewComment>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let query = query.0;
    let new_comment = data.0;
    let conn = &mut connect_to_db();

    // Check if parent review exists
    let review_result = reviews::table
        .select(Review::as_select())
        .find(query.id)
        .first::<Review>(conn);
    if review_result.is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid review id")
    }
    let review_id = review_result.unwrap().id;

    // Check if parent comment is given, and if so if it exists
    let parent_id_parsed: Result<Option<i32>, ()> = match new_comment.parent_id {
        Some(parent_id) => {
            match comments::table
                .select(Comment::as_select())
                .filter(comments::review_id.eq(query.id))
                .filter(comments::id.eq(parent_id))
                .first::<Comment>(conn) {
                    Ok(parent) => Ok(Some(parent.id)),
                    Err(_) => Err(()),
                }
        },
        None => Ok(None),
    };
    if parent_id_parsed.is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid parent id")
    }
    let parent_id = parent_id_parsed.unwrap();

    let values = (
        comments::user_id.eq(user.user_id()),
        comments::review_id.eq(review_id),
        comments::comment.eq(new_comment.comment),
        comments::comment_id.eq(parent_id),
    );

    // Try to create the comment
    let insert_result = insert_into(comments::table)
        .values(values)
        .execute(conn);
    if insert_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Comment creation failed")
    }
    
    // Comment creation succeeded
    (StatusCode::OK, "Comment created")
}

// Handles DELETE to delete a specific comment
async fn delete_comment(ctx: Result<Ctx, String>, query: Query<IdQuery>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let query = query.0;
    let conn = &mut connect_to_db();

    // Get comment
    let comment_query = comments::table
        .select(Comment::as_select())
        .find(query.id)
        .first::<Comment>(conn);
    if comment_query.is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid comment id".to_string())
    }
    let comment_to_delete = comment_query.unwrap();

    // Don't allow users to delete other people's comments (except admins)
    if comment_to_delete.user_id != user.user_id() && !user.is_admin() {
        return (StatusCode::FORBIDDEN, "You may not delete comments written by other users".to_string())
    }

    // Look for children
    let child_query = Comment::belonging_to(&comment_to_delete)
        .select(Comment::as_select())
        .load::<Comment>(conn);
    if child_query.is_err() {
        return (StatusCode::BAD_REQUEST, format!("Error getting children: {}", child_query.unwrap_err().to_string()))
    }
    let children = child_query.unwrap();
    // If there are children, just mark comment as deleted instead of removing it
    let result = match children.len() > 0 {
        true => {
            dsl::update(comments::table.find(query.id))
                .set(comments::comment.eq("[ REMOVED ]"))
                .execute(conn)
        },
        false => {
            // No children, just delete the comment
            dsl::delete(comments::table.find(query.id))
                .execute(conn)
        },
    };
    
    // Match result into response
    match result {
        Ok(_) => (StatusCode::OK, "Comment removed".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
