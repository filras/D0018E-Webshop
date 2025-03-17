use crate::{
    auth::{self, ctx::Ctx},
    db::{connect_to_db, models::Order},
    schema::orders,
};
use axum::{
    extract::Json,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, delete},
    Router,
};
use management::{cancel_order, create_order};
use serde::Deserialize;
use diesel::{prelude::*, result::Error};
use tsync::tsync;

pub mod management;

pub fn routes() -> Router {
    Router::new()
        .route("/order/pending", get(handle_get_pending))
        .route("/order/create", post(handle_create_order))
        .route("/order/complete", post(handle_complete_order))
        .route("/order/cancel", delete(handle_cancel_order))
        .layer(middleware::from_fn(auth::middleware::require_auth))
}

#[derive(Deserialize)]
#[tsync]
pub struct ShippingInformation {
    address: String,
    co: Option<String>,
    zipcode: String,
    country: String,
    comment: Option<String>,
}

// Get current pending order for authed user
async fn handle_get_pending(ctx: Result<Ctx, String>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let user = ctx.unwrap();
    
    // Check if user has pending order
    let ongoing_order_query = orders::table
        .filter(orders::user_id.eq(user.user_id()))
        .filter(orders::payment_completed.eq(false))
        .select(Order::as_select())
        .first::<Order>(conn);
    if ongoing_order_query.is_err() {
        let error = ongoing_order_query.unwrap_err();
        
        if error == Error::NotFound {
            return (StatusCode::BAD_REQUEST, format!("User {} has no pending order", user.user_id())).into_response();
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read order for user {}: {}", user.user_id(), error)).into_response();
        }
    }

    Json(ongoing_order_query.unwrap()).into_response()
}

// Create a new order (start checkout)
async fn handle_create_order(
    ctx: Result<Ctx, String>,
    data: Json<ShippingInformation>,
) -> impl IntoResponse {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();
    let shipping_info = data.0;

    let create_order_result = create_order(conn, user.user_id(), shipping_info);
    if create_order_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create order: {}", create_order_result.unwrap_err())).into_response();
    }

    let order_id = create_order_result.unwrap();
    (StatusCode::OK, order_id.to_string()).into_response()
}

// Get current pending order for authed user
async fn handle_complete_order(ctx: Result<Ctx, String>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let user = ctx.unwrap();
    
    // Get id of the user's pending order
    let orderid_result = orders::table
        .filter(orders::user_id.eq(user.user_id()))
        .filter(orders::payment_completed.eq(false))
        .select(orders::id)
        .first::<i32>(conn);
    if orderid_result.is_err() {
        let error = orderid_result.unwrap_err();
        
        if error == Error::NotFound {
            return (StatusCode::BAD_REQUEST, format!("User {} has no pending order", user.user_id())).into_response();
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read order for user {}: {}", user.user_id(), error)).into_response();
        }
    }
    let order_id = orderid_result.unwrap();

    // Update order to mark it as completed
    match diesel::update(orders::table)
        .filter(orders::id.eq(order_id))
        .set(orders::payment_completed.eq(true))
        .execute(conn)
    {
        Ok(_) => format!("Completed order #{}", order_id).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to complete order #{}: {}", order_id, e.to_string())).into_response(),
    }
}

// Cancel currently pending order for authed user
async fn handle_cancel_order(ctx: Result<Ctx, String>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let user = ctx.unwrap();
    
    // Check if user has pending order
    let ongoing_order_query = orders::table
        .filter(orders::user_id.eq(user.user_id()))
        .filter(orders::payment_completed.eq(false))
        .select(orders::id)
        .first::<i32>(conn);
    if ongoing_order_query.is_err() {
        let error = ongoing_order_query.unwrap_err();
        
        if error == Error::NotFound {
            return (StatusCode::BAD_REQUEST, format!("User {} has no pending order", user.user_id())).into_response();
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read order for user {}: {}", user.user_id(), error)).into_response();
        }
    }
    let order_id = ongoing_order_query.unwrap();

    let cancel_order_result = cancel_order(conn, order_id);
    if cancel_order_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to cancel order #{}: {}", order_id, cancel_order_result.unwrap_err())).into_response();
    }

    format!("Order #{} cancelled", order_id).into_response()
}
