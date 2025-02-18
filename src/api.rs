use self::models::*;
#[allow(unused)]
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use diesel::{dsl::insert_into, query_builder::IncompleteInsertOrIgnoreStatement, SelectableHelper};
use diesel::{prelude::*, QueryResult};
use schema::items::{average_rating, description, discounted_price, in_stock, title};
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_writer_pretty;
use std::fs::File;
use std::path::Path;
use D0018E_Webshop::*;

fn default_page() -> usize {
    1
}
fn default_per_page() -> usize {
    10
}
#[derive(Debug, Deserialize)]
struct Pagination {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_per_page")]
    per_page: usize,
}

// routes for api

pub fn routes() -> Router {
    Router::new().route("/items", get(get_items))
}

pub async fn get_items(pagination: Query<Pagination>) -> impl IntoResponse {
    let pagination: Pagination = pagination.0;
    use self::schema::items::dsl::*;
    let conn = &mut connect_to_db();
    let results: Vec<Item> = items
        .offset(((pagination.page - 1) * pagination.per_page) as i64)
        .limit(pagination.per_page as i64)
        .select(Item::as_select())
        .load::<Item>(conn)
        .expect("Error loading items");
    (StatusCode::OK, Json(results))
}

pub fn create_item( conn: &mut MysqlConnection, Vec) -> Item {
    
   }

async fn post_items(conn: &mut MysqlConnection, data: Json<Item>) -> impl IntoResponse {
     let rcv_item: Item = data.0;
     use schema::items::dsl::*;

    insert_into(items)
        .values((
            title.eq(rcv_item.title),
            description.eq(rcv_item.description),
            price.eq(rcv_item.price),
            in_stock.eq(rcv_item.in_stock),
            average_rating.eq(rcv_item.average_rating),
            discounted_price.eq(rcv_item.discounted_price)))
        .execute(conn)
        
    (StatusCode::OK, "Item recieved")
}
