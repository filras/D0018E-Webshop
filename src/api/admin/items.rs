#[allow(unused)]
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use diesel::{
    dsl::{delete, insert_into, update}, prelude::*
};

use crate::{
    db::{
        connect_to_db,
        models::{IdQuery, NewItem, UpdateItem},
    }, schema::items::{dsl::items, *}
};

pub fn routes() -> Router {
    Router::new()
        .route("/items", 
            post(handle_post)
            .put(handle_put)
            .delete(handle_delete))
}

// Create a new item
async fn handle_post(data: Json<NewItem>) -> impl IntoResponse {
    let rcv_item: NewItem = data.0;
    let conn = &mut connect_to_db();
    let values = (
        title.eq(rcv_item.title),
        description.eq(rcv_item.description),
        price.eq(rcv_item.price),
        in_stock.eq(rcv_item.in_stock),
        average_rating.eq(rcv_item.average_rating),
        discounted_price.eq(rcv_item.discounted_price),
    );

    return match insert_into(items)
        .values(values)
        .execute(conn) {
        Ok(_) => (StatusCode::OK, "Item recieved".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

// Edit an item
async fn handle_put(item: Query<IdQuery>, data: Json<UpdateItem>) -> impl IntoResponse {
    let rcv_item: UpdateItem = data.0;
    let item_id = item.0.id;
    
    let conn = &mut connect_to_db();
    return match update(items)
    .filter(id.eq(item_id))
    .set(rcv_item)
    .execute(conn)
    {
        Ok(items_updated) => match items_updated > 0 {
            true => (StatusCode::OK, format!("Item {} updated", item_id)).into_response(),
            false => (StatusCode::BAD_REQUEST, "No item found").into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
}

// Delete an item
async fn handle_delete(item: Query<IdQuery>) -> impl IntoResponse {
    let conn = &mut connect_to_db();
    let item_id = item.0.id;

    // Try to delete the item
    let result = delete(items.filter(id.eq(item_id))).execute(conn);
    
    // Return result
    match result {
        Ok(items_deleted) => match items_deleted > 0 {
            true => (StatusCode::OK, "Item deleted"),
            false => (StatusCode::BAD_REQUEST, "No item found"),
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Item deletion failed"),
    }
}
