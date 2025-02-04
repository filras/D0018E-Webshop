#[allow(unused)]
use axum::{
    http::StatusCode, 
    Router, 
    routing::get, 
    response::{IntoResponse, Response},  
    extract::{Query, Json}};
use serde::Serialize;
use std::fs::File;
use std::path::Path;
use serde::{de, Deserialize, Deserializer};
use serde_json::{to_writer_pretty};
use std::{fmt, str::FromStr};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
id: u32,
title: String,
description : Option<String>,
price: String,
in_stock: u32,
average_rating: Option<f32>
}

fn default_page() -> usize {
    1
}
fn default_per_page () -> usize {
    10
}
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_per_page")]
    per_page: usize

}

async fn write_json(to_write: Item) {
    let file_path = Path::new("./MockData.json");
    let file = File::open(file_path).expect("file not found");
    to_writer_pretty(file, &to_write);

}

async fn read_json () -> Vec<Item>{

    let file_path = Path::new("./MockData.json");
    let file = File::open(file_path).expect("file not found");
    let items:Vec<Item> = serde_json::from_reader(file).expect("error while parsing");
    return items

}
// routes for api
//async fn main(){

//let api = Router::new()
//   .route("/items", get(get_items).post(post_items));


//}

async fn paginate_vec(v: Vec<Item>, page: usize, per_page: usize) -> Vec<Item>{
   let start = (page-1) * per_page;
   let mut end = start + per_page;
   if end > v.len(){
        end = v.len();
   }
   let res = v[start..end].to_vec();
   

    return res;

}

pub async fn get_items(pagination: Query<Pagination>) -> impl IntoResponse { 
    let pagination: Pagination = pagination.0;
    let items:Vec<Item> = read_json().await;
    let result:Vec<Item> = paginate_vec(items, pagination.page, pagination.per_page).await;
    (StatusCode::OK, Json(result))        
}

pub async fn post_items(data: Json<Item>) -> impl IntoResponse {
    let rcv_item: Item = data.0;
    println! ("Recieved Item {:?}", rcv_item);
    write_json(rcv_item).await;
    (StatusCode::OK, "Item recieved")
    
}
