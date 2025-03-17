use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, delete},
    Router,
};

use diesel::{self, prelude::*, result::Error};
use serde::Serialize;
use tsync::tsync;

use crate::{
    api::order::management::{cancel_order, remove_order},
    db::{
        connect_to_db,
        models::{IdQuery, Order, PaginatedSearchQuery},
    },
    schema::{self, items, order_items, orders, users}
};

pub fn routes() -> Router {
    Router::new()
        .route("/orders", get(handle_get_orders))
        .route("/order", get(handle_get_order_data))
        .route("/order/cancel", delete(handle_cancel_order))
        .route("/order/remove", delete(handle_remove_order))
}

#[derive(Serialize, Queryable, Debug)]
#[tsync]
struct OrderWithUserData {
    pub id: i32,
    pub address: String,
    pub zipcode: String,
    pub co: Option<String>,
    pub country: String,
    pub comment: Option<String>,
    pub total: i32,
    pub payment_completed: bool,
    
    pub username: String,
    pub firstname: String,
    pub surname: String,
    pub email: String,
}

// Columns for 
const COLUMNS_FOR_ORDER_WITH_USERDATA: (
    schema::orders::columns::id,
    schema::orders::columns::address,
    schema::orders::columns::zipcode,
    schema::orders::columns::co,
    schema::orders::columns::country,
    schema::orders::columns::comment,
    schema::orders::columns::total,
    schema::orders::columns::payment_completed,
    schema::users::columns::username,
    schema::users::columns::firstname,
    schema::users::columns::surname,
    schema::users::columns::email
) = (
    orders::id,
    orders::address,
    orders::zipcode,
    orders::co,
    orders::country,
    orders::comment,
    orders::total,
    orders::payment_completed,
    
    users::username,
    users::firstname,
    users::surname,
    users::email,
);


// Query for orders, uses search (for username) if present, otherwise all. Paginated
async fn handle_get_orders(query: Query<PaginatedSearchQuery>) -> impl IntoResponse {
    let query: PaginatedSearchQuery = query.0;
    let conn = &mut connect_to_db();

    // Make different queries depending on if we're searching for username
    let query_results = match query.search {
        // Include only results filtered with search_string on username
        // We perform a join on the order with user so we can search by username
        Some(search_string) => orders::table
            .inner_join(users::table)
            .filter(users::username.like(format!("%{}%", search_string)))
            .offset(((query.page - 1) * query.per_page) as i64)
            .limit(query.per_page as i64)
            .select(COLUMNS_FOR_ORDER_WITH_USERDATA)
            .load::<OrderWithUserData>(conn),
        // Include all paginated results
        None => orders::table
            .offset(((query.page - 1) * query.per_page) as i64)
            .limit(query.per_page as i64)
            .inner_join(users::table)
            .select(COLUMNS_FOR_ORDER_WITH_USERDATA)
            .load::<OrderWithUserData>(conn),
    };

    // Make results into response
    match query_results {
        Ok(results) => (StatusCode::OK, Json(results)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Serialize, Queryable, Debug)]
#[tsync]
struct OrderWithUserDataAndItems {
    pub id: i32,
    pub address: String,
    pub zipcode: String,
    pub co: Option<String>,
    pub country: String,
    pub comment: Option<String>,
    pub total: i32,
    pub payment_completed: bool,
    
    pub username: String,
    pub firstname: String,
    pub surname: String,
    pub email: String,

    pub items: Vec<OrderItemWithData>
}

#[derive(Serialize, Queryable, Debug)]
#[tsync]
struct OrderItemWithData {
    pub name: String,
    pub price: i32,
    pub discounted_price: Option<i32>,
    pub amount: i32,
    pub total: i32,
}

// Get compounded JSON about a single order and its order items
async fn handle_get_order_data(query: Query<IdQuery>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let order_query: IdQuery = query.0;

    // Get order data
    let order_data_res = orders::table
        .find(order_query.id)
        .inner_join(users::table)
        .select(COLUMNS_FOR_ORDER_WITH_USERDATA)
        .first::<OrderWithUserData>(conn);
    if order_data_res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, order_data_res.unwrap_err().to_string()).into_response()
    }
    let order_data = order_data_res.unwrap();

    // Get order items
    let order_items_res = order_items::table
        .filter(order_items::order_id.eq(order_data.id))
        .inner_join(items::table)
        .select((
            items::title,
            items::price,
            items::discounted_price,
            order_items::amount,
            order_items::total,
        ))
        .load::<OrderItemWithData>(conn);
    if order_items_res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, order_items_res.unwrap_err().to_string()).into_response()
    }

    // Return order data with items as JSON
    Json(OrderWithUserDataAndItems {
        id:  order_data.id,
        address:  order_data.address,
        zipcode:  order_data.zipcode,
        co: order_data.co,
        country:  order_data.country,
        comment: order_data.comment,
        total:  order_data.total,
        payment_completed:  order_data.payment_completed,
    
        username: order_data.username,
        firstname: order_data.firstname,
        surname: order_data.surname,
        email: order_data.email,

        items: order_items_res.unwrap(),
    }).into_response()
}

// Cancel any order by order ID (this works only for pending orders)
async fn handle_cancel_order(query: Query<IdQuery>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let order_id = query.0.id;

    // Check if order exists and is pending
    let ongoing_order_query = orders::table
        .find(order_id)
        .select(Order::as_select())
        .first::<Order>(conn);
    if ongoing_order_query.is_err() {
        let error = ongoing_order_query.unwrap_err();
        
        if error == Error::NotFound {
            return (StatusCode::BAD_REQUEST, format!("Order #{} doesn't exist", order_id)).into_response();
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read order #{}: {}", order_id, error)).into_response();
        }
    }
    
    // Check so order is actually pending (not yet completed)
    let order = ongoing_order_query.unwrap();
    if order.payment_completed == true {
        return (StatusCode::BAD_REQUEST, format!("Order #{} is already completed", order_id)).into_response();
    }


    let cancel_order_result = cancel_order(conn, order_id);
    if cancel_order_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to cancel order #{}: {}", order_id, cancel_order_result.unwrap_err())).into_response();
    }

    format!("Order #{} cancelled", order_id).into_response()
}

// Remove any order by order ID (this works only for completed orders)
async fn handle_remove_order(query: Query<IdQuery>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let order_id = query.0.id;

    // Check if order exists and isn't pending
    let order_query = orders::table
        .find(order_id)
        .select(Order::as_select())
        .first::<Order>(conn);
    if order_query.is_err() {
        let error = order_query.unwrap_err();
        
        if error == Error::NotFound {
            return (StatusCode::BAD_REQUEST, format!("Order #{} doesn't exist", order_id)).into_response();
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read order #{}: {}", order_id, error)).into_response();
        }
    }
    
    // Check so order is not pending (payment completed)
    let order = order_query.unwrap();
    if order.payment_completed == false {
        return (StatusCode::BAD_REQUEST, format!("Order #{} is still pending, use /cancel to remove it", order_id)).into_response();
    }

    let cancel_order_result = remove_order(conn, order_id);
    if cancel_order_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to remove order #{}: {}", order_id, cancel_order_result.unwrap_err())).into_response();
    }

    format!("Order #{} removed", order_id).into_response()
}
