use crate::{
    auth::{self, ctx::Ctx},
    db::{
        connect_to_db,
        models::{CartItems, IdQuery, User},
    },
    schema::{
        cart_items::{self, amount, item_id, user_id},
        items, users,
    },
};
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
use serde::{Deserialize, Serialize};
use tsync::tsync;

pub fn routes() -> Router {
    Router::new()
        .route("/cart", get(get_cart).put(put_cart).delete(delete_cart))
        .layer(middleware::from_fn(auth::middleware::require_auth))
}

// Struct used to return cart and item join as one object
#[derive(Serialize, Queryable)]
#[tsync]
struct CombinedCartItem {
    item_id: i32,
    title: String,
    description: Option<String>,
    price: i32,
    in_stock: i32,
    average_rating: Option<f32>,
    discounted_price: Option<i32>,
    amount: i32,
}

async fn get_cart(ctx: Result<Ctx, String>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();

    let user_obj: User = users::table.find(user.user_id()).first(conn).unwrap();

    let result = CartItems::belonging_to(&user_obj)
        .inner_join(items::table.on(items::id.eq(cart_items::item_id)))
        .select((
            items::id,
            items::title,
            items::description,
            items::price,
            items::in_stock,
            items::average_rating,
            items::discounted_price,
            cart_items::amount,
        ))
        .load::<CombinedCartItem>(conn)
        .unwrap();

    return (StatusCode::OK, Json(result).into_response());
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = cart_items)]
#[tsync]
struct UpdateCart {
    item_id: i32,
    amount: i32,
}
async fn put_cart(ctx: Result<Ctx, String>, data: Json<UpdateCart>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let rcv_items: UpdateCart = data.0;
    let conn = &mut connect_to_db();

    // Deletes item if given amount <= 0
    if rcv_items.amount <= 0 {
        let result = diesel::delete(
            cart_items::table
                .filter(user_id.eq(user.user_id()))
                .filter(item_id.eq(rcv_items.item_id)),
        )
        .execute(conn);
        if result.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while deleteing item",
            )
                .into_response();
        }
        return (StatusCode::OK, "Item deleted").into_response();
    }

    let values = (
        user_id.eq(user.user_id()),
        item_id.eq(rcv_items.item_id),
        amount.eq(rcv_items.amount),
    );

    // Check if recived item is already in the cart and updates the cart if that is the case
    let item_in_cart = cart_items::table
        .filter(user_id.eq(user.user_id()))
        .filter(item_id.eq(rcv_items.item_id))
        .select(CartItems::as_select())
        .first::<CartItems>(conn);
    if item_in_cart.is_ok() {
        let result = diesel::update(cart_items::table)
            .filter(user_id.eq(user.user_id()))
            .filter(item_id.eq(rcv_items.item_id))
            .set(values)
            .execute(conn);
        if result.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while adding to cart",
            )
                .into_response();
        }

        return (StatusCode::OK, "Cart updated").into_response();
    }

    // Insert a new item into the cart
    let result = diesel::insert_into(cart_items::table)
        .values(values)
        .execute(conn);
    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while adding to cart",
        )
            .into_response();
    }

    return (StatusCode::OK, "Item added").into_response();
}

async fn delete_cart(ctx: Result<Ctx, String>, id_query: Query<IdQuery>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();
    let rcv_id: IdQuery = id_query.0;

    let result = diesel::delete(
        cart_items::table
            .filter(user_id.eq(user.user_id()))
            .filter(item_id.eq(rcv_id.id)),
    )
    .execute(conn);
    if result.is_ok() {
        return (StatusCode::OK, "Item deleted");
    }

    return (StatusCode::INTERNAL_SERVER_ERROR, "Item deletion failed");
}
