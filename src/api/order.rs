use crate::{
    auth::{self, ctx::Ctx},
    db::{
        connect_to_db,
        models::{CartItems, IdQuery, Item, OrderItems, Order},
    },
    schema::{
        cart_items::{self, user_id},
        items::{self, in_stock},
        order_items::{self, order_id},
        orders::{self},
    },
};
use axum::routing::{delete, post};
#[allow(unused)]
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use diesel::prelude::*;
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/order/create", post(create_order))
        .layer(middleware::from_fn(auth::middleware::require_auth))
        .route("/order/cancel", delete(cancel_order))
        .layer(middleware::from_fn(auth::middleware::require_auth))
}

#[derive(Deserialize)]
struct ShippingInformation {
    address: String,
    co: Option<String>,
    zipcode: String,
    country: String,
    comment: Option<String>,
}

async fn reserve_items(oid: i32) -> std::result::Result<(), ()> {
    let conn = &mut connect_to_db();
    let reserve_items = order_items::table
        .filter(order_id.eq(oid))
        .select(OrderItems::as_select())
        .load::<OrderItems>(conn)
        .unwrap();

    for o in reserve_items {
        let cmp_item = items::table
            .find(o.item_id)
            .select(Item::as_select())
            .first::<Item>(conn)
            .unwrap();

        if cmp_item.in_stock < o.amount {
            return Err(());
        }

        let item = diesel::update(items::table)
            .filter(items::id.eq(o.item_id))
            .set(items::in_stock.eq(in_stock - o.amount))
            .execute(conn);

        if item.is_err() {
            return Err(());
        }
    }
    return Ok(());
}

async fn create_order(
    ctx: Result<Ctx, String>,
    data: Json<ShippingInformation>,
) -> impl IntoResponse {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();
    let rcv_info: ShippingInformation = data.0;

    let cart = cart_items::table
        .filter(user_id.eq(user_id))
        .select(CartItems::as_select())
        .load::<CartItems>(conn)
        .unwrap();

    let mut total = 0;

    for n in cart {
        let item_total = items::table
            .filter(items::id.eq(n.item_id))
            .select(Item::as_select())
            .first::<Item>(conn)
            .unwrap();

        let price = match item_total.discounted_price {
            Some(price) => price,
            None => item_total.price,
        };

        total += price * n.amount;
    }

    let values = (
        orders::user_id.eq(user.user_id()),
        orders::address.eq(rcv_info.address),
        orders::co.eq(rcv_info.co),
        orders::zipcode.eq(rcv_info.zipcode),
        orders::country.eq(rcv_info.country),
        orders::total.eq(total),
        orders::comment.eq(rcv_info.comment),
        orders::payment_completed.eq(false),
    );
    // Check if user has a current ongoing order
    let ongoing_order = orders::table
        .filter(orders::user_id.eq(user.user_id()))
        .select(Order::as_select())
        .first::<Order>(conn);
    if ongoing_order.is_ok() {
        return (
            StatusCode::BAD_REQUEST,
            "User already has an order in progress",
        )
            .into_response();
    }

    let result = diesel::insert_into(orders::table)
        .values(values)
        .execute(conn);

    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while creating order 1",
        )
            .into_response();
    }

    let oid_order = orders::table
        .filter(orders::user_id.eq(user.user_id()))
        .select(Order::as_select())
        .first::<Order>(conn)
        .unwrap();

    let oid = oid_order.id;
    let cart2 = cart_items::table
        .filter(user_id.eq(user_id))
        .select(CartItems::as_select())
        .load::<CartItems>(conn)
        .unwrap();

    for n in cart2 {
        let item_price = items::table
            .filter(items::id.eq(n.item_id))
            .select(Item::as_select())
            .first::<Item>(conn)
            .unwrap();

        let price = match item_price.discounted_price {
            Some(price) => price,
            None => item_price.price,
        };

        let cart_values = (
            order_items::order_id.eq(oid),
            order_items::item_id.eq(n.item_id),
            order_items::amount.eq(n.amount),
            order_items::total.eq(price * n.amount),
        );

        let res = diesel::insert_into(order_items::table)
            .values(cart_values)
            .execute(conn);
        if res.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while creating order 2",
            )
                .into_response();
        }

        let del = diesel::delete(cart_items::table)
            .filter(cart_items::user_id.eq(user.user_id()))
            .filter(cart_items::item_id.eq(n.item_id))
            .execute(conn);
        if del.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while creating order 3",
            )
                .into_response();
        }
    }

    let reserve_res = reserve_items(oid).await;
    if reserve_res.is_err() {
        return (StatusCode::BAD_REQUEST, "Error while reserving order items").into_response();
    }

    //TODO add timer and start it
    return (StatusCode::OK, Json(oid)).into_response();
}

async fn release_items(oid: i32) {
    let conn = &mut connect_to_db();
    let o_items = order_items::table
        .filter(order_id.eq(oid))
        .select(OrderItems::as_select())
        .load::<OrderItems>(conn)
        .unwrap();

    for o in o_items {
        let rel = diesel::update(items::table)
            .filter(items::id.eq(o.item_id))
            .set(items::in_stock.eq(in_stock + o.amount))
            .execute(conn);
        if rel.is_err() {
            return;
        }
    }
}

pub async fn cancel_order(oid: Query<IdQuery>) -> impl IntoResponse {
    //TODO remove timer
    let conn = &mut connect_to_db();
    let ord_id: IdQuery = oid.0;
    let res = diesel::delete(order_items::table)
        .filter(order_items::order_id.eq(ord_id.id))
        .execute(conn);
    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "order_item deletion failed",
        )
            .into_response();
    }

    let result = diesel::delete(orders::table.find(ord_id.id)).execute(conn);

    if result.is_ok() {
        release_items(ord_id.id).await;
        return (StatusCode::OK, "Order deleted").into_response();
    }

    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        result.unwrap_err().to_string(),
    )
        .into_response();
    //return (StatusCode::INTERNAL_SERVER_ERROR, "Order deletetion failed").into_response();
}
