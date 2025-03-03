use crate::{
    auth::{self, ctx::Ctx, session},
    db::{
        connect_to_db,
        models::{CartItems, Item, User},
    },
    schema::{
        cart_items::{self, user_id},
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
use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/cart", get(get_cart).put(put_cart))
        .layer(middleware::from_fn(auth::middleware::require_auth))
}

async fn get_cart(ctx: Result<Ctx, String>) -> impl IntoResponse {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();

    let user_obj: User = users::table.find(user.user_id()).first(conn).unwrap();

    // .filter(id.eq(user.user_id()))
    //  .select(User::as_select())
    // .first::<User>(conn);

    let result: Vec<Item> = CartItems::belonging_to(&user_obj)
        .inner_join(items::table)
        .select(Item::as_select())
        .load(conn)
        .unwrap();
    (StatusCode::OK, Json(result).into_response())
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

    return match diesel::update(cart_items::table)
        .filter(user_id.eq(user.user_id()))
        .set(rcv_items)
        .execute(conn)
    {
        Ok(_) => (StatusCode::OK, "Cart updated").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
}
