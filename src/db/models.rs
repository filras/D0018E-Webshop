use crate::schema::*;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

// Tsync syncs types from Rust to the frontend in TS
// To perform a sync, add #[tsync] to the struct and sync with `cargo run --bin tsync`
use tsync::tsync;

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[tsync]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub in_stock: i32,
    pub average_rating: Option<f32>,
    pub discounted_price: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = items)]
#[tsync]
pub struct NewItem {
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub in_stock: i32,
    pub average_rating: Option<f32>,
    pub discounted_price: Option<i32>,
}

// Having Options here means we will automatically ignore any fields not included in the query instead of writing these as null
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = items)]
#[tsync]
pub struct UpdateItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub in_stock: Option<i32>,
    pub average_rating: Option<f32>,
    pub discounted_price: Option<i32>,
}

#[derive(
    AsChangeset,
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[tsync]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub firstname: String,
    pub surname: String,
    pub email: String,
    pub role: String,
    pub address: Option<String>,
    pub zipcode: Option<String>,
    pub co: Option<String>,
    pub country: Option<String>,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, PartialEq, Eq, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Item))]
#[diesel(table_name = cart_items)]
#[diesel(primary_key(user_id, item_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[tsync]
pub struct CartItems {
    pub user_id: i32,
    pub item_id: i32,
    pub amount: i32,
}
// User for register
#[derive(Deserialize)]
#[tsync]
pub struct NewUser {
    pub password: String,
    pub firstname: String,
    pub surname: String,
    pub email: String,
}

// Having Options here means we will automatically ignore any fields not included in the query instead of writing these as null
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
#[tsync]
pub struct UpdateUser {
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub address: Option<String>,
    pub zipcode: Option<String>,
    pub co: Option<String>,
    pub country: Option<String>,
}

// Having Options here means we will automatically ignore any fields not included in the query instead of writing these as null
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
#[tsync]
pub struct UpdateUserAsAdmin {
    pub username: Option<String>,
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub address: Option<String>,
    pub zipcode: Option<String>,
    pub co: Option<String>,
    pub country: Option<String>,
}

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    Associations,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
)]
#[diesel(table_name = reviews)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Item))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[tsync]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub comment: Option<String>,
    pub rating: i32,
}

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    Associations,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
)]
#[diesel(table_name = comments)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Review))]
#[diesel(belongs_to(Comment))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[tsync]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub review_id: i32,
    pub comment_id: Option<i32>,
    pub comment: String,
}

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    Associations,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
)]
#[diesel(table_name = order_items)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Item))]
#[diesel(primary_key(order_id, item_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[tsync]
pub struct OrderItems {
    pub order_id: i32,
    pub item_id: i32,
    pub amount: i32,
    pub total: i32,
}

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[tsync]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
    pub co: Option<String>,
    pub zipcode: String,
    pub country: String,
    pub total: i32,
    pub comment: Option<String>,
    pub payment_completed: bool,
}

// Generic query by ID
#[derive(Deserialize)]
pub struct IdQuery {
    pub id: i32,
}

// Generic paginated search query struct
fn default_page() -> usize {
    1
}
fn default_per_page() -> usize {
    10
}

#[derive(Debug, Deserialize)]
pub enum SortBy {
    Name,
    Price,
}

#[derive(Debug, Deserialize)]
#[tsync]
pub struct PaginatedSearchQuery {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,

    pub sort_by: Option<SortBy>,
    pub search: Option<String>,
}

// Generic paginated id query (query by id)
#[derive(Debug, Deserialize)]
#[tsync]
pub struct PaginatedIdQuery {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,

    pub id: i32,
}
