use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use diesel::{self, prelude::*};
use serde::Serialize;
use tsync::tsync;

use crate::{
    api::order::release_items,
    db::{
        connect_to_db,
        models::{IdQuery, PaginatedSearchQuery},
    },
    schema::{self, items, order_items, orders, users},
};

pub fn routes() -> Router {
    Router::new()
        .route("/orders", get(handle_get))
        .route("/order_data", get(get_order_data))
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
async fn handle_get(query: Query<PaginatedSearchQuery>) -> impl IntoResponse {
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

async fn cancel_order(query: Query<IdQuery>) -> impl IntoResponse {
    //TODO remove timer
    let conn = &mut connect_to_db();
    let order_query: IdQuery = query.0;
    let res = diesel::delete(order_items::table)
        .filter(order_items::order_id.eq(order_query.id))
        .execute(conn);
    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "order_item deletion failed",
        )
            .into_response();
    }

    let result = diesel::delete(orders::table.find(order_query.id)).execute(conn);

    if result.is_ok() {
        release_items(order_query.id).await;
        return (StatusCode::OK, "Order deleted").into_response();
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        result.unwrap_err().to_string(),
    )
        .into_response();
    //return (StatusCode::INTERNAL_SERVER_ERROR, "Order deletetion failed").into_response();
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

async fn get_order_data(query: Query<IdQuery>) -> impl IntoResponse {
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
