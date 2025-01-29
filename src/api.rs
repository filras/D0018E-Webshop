#[allow(unused)]
use axum::{http::StatusCode, Router, routing::get, response::{IntoResponse, Response}, Json, extract::Path};
use serde::Serialize;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
struct Item {
id: u32,
title: String,
description : Option<String>,
price: String,
in_stock: u32,
average_rating: Option<f32>
}

#[derive(Deserialize)]
struct Pagination {
    skip: u32,
    limit: u32

}



async fn read_json () -> Vec<Item>{

let file_path = std::Path::new("./MockData.json");
let file = File::open(file_path).expect("file not found");
let items:Vec<Item> = serde_json::from_reader(file).expect("error while parsing");
return items

}
// routes for api
//async fn main(){

//let api = Router::new()
//   .route("/items", get(get_items).post(post_items));


//}

async fn floor_vec(v: Vec<Item>, floor: i32) -> Vec<Item>{
    let mut res:Vec<Item>;
    for i in floor..v.len() as i32{
        res.push(v[i as usize]);
    }

    return res;

}

pub async fn get_items(Path((skip)): Path<(u32)>,) -> impl IntoResponse { 
    let items:Vec<Item> = read_json().await;
    let page = &items[1..items.len()];
    (StatusCode::OK, Json(page))
}

async fn post_items(){

}
