use crate::{
    auth::{self, ctx::Ctx},
    db::{
        connect_to_db,
        models::{IdQuery, Order, OrderItems},
    },
    schema::{order_items::*, orders::*},
};
use axum::routing::post;
#[allow(unused)]
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use diesel::{dsl::select, prelude::*};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

pub 

pub fn routes() -> Router {
    Router::new()
        .route("/order/create", post(create_order))
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

async fn create_order(
    ctx: Result<Ctx, String>,
    data: Json<ShippingInformation>,
    cart_id: Query<IdQuery>,
) {
    let user = ctx.unwrap();
    let conn = &mut connect_to_db();
    let rcv_id: IdQuery = cart_id.0;
    let rcv_info: ShippingInformation = data.0;

    let values = (
        user_id.eq(user.user_id()),
        address.eq(rcv_info.address),
        co.eq(rcv_info.co),
        zipcode.eq(rcv_info.zipcode),
        country.eq(rcv_info.country),
        comment.eq(rcv_info.comment),
        payment_completed.eq(false),
    );
    // Check if user has a current ongoing order
    let ongoing_order = orders::table
        .filter(user_id.eq(user.user_id()))
        .select(Order::as_select())
        .first::<Order>(conn);
    if ongoing_order.is_ok() {
        return;
    }

    let result = diesel::insert_into(orders::table)
        .values(values)
        .execute(conn);

    let runtime = Runtime::new().unwrap();
    let (cancel)
    runtime.block_on(async {
        let s = sleep(Duration::from_secs(100)).await;
        cancel_order(ctx, query);
    });
}

async fn cancel_order(ctx: Result<Ctx, String>, data: Query<IdQuery>) -> impl IntoResponse {}
