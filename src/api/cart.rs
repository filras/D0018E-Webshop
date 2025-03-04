use crate::{
    auth::{self, ctx::Ctx},
    db::{
        connect_to_db,
        models::{CartItems, User},
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

pub fn routes() -> Router {
    Router::new()
        .route("/cart", get(get_cart).put(put_cart))
        .layer(middleware::from_fn(auth::middleware::require_auth))
}

#[derive(Serialize, Queryable)]
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

    // .filter(id.eq(user.user_id()))
    //  .select(User::as_select())
    // .first::<User>(conn);

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
struct UpdateCart {
    item_id: i32,
    amount: i32,
}
async fn put_cart(ctx: Result<Ctx, String>, data: Json<UpdateCart>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let rcv_items: UpdateCart = data.0;
    let conn = &mut connect_to_db();

    let values = (
        user_id.eq(user.user_id()),
        item_id.eq(rcv_items.item_id),
        amount.eq(rcv_items.amount),
    );

    // Insert new user into DB
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

    // return match diesel::update(cart_items::table)
    //    .filter(user_id.eq(user.user_id()))
    //    .set(rcv_items)
    //    .execute(conn)
    //   {
    //    Ok(_) => (StatusCode::OK, "Cart updated").into_response(),
    //    Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    //bin.usr-is-merged  };
    return (StatusCode::OK, "cart updated").into_response();
}
