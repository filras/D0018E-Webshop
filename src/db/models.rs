use crate::schema::*;
use diesel::prelude::*;
use diesel::Queryable;
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
pub struct Item {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub in_stock: i32,
    pub average_rating: Option<f32>,
    pub discounted_price: Option<i32>,
}

#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub in_stock: i32,
    pub average_rating: Option<f32>,
    pub discounted_price: Option<i32>,
}

#[derive(
    AsChangeset,
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
    Eq,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
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

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, PartialEq, Eq)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Item))]
#[diesel(table_name = cart_items)]
#[diesel(primary_key(user_id, item_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct CartItems {
    pub user_id: i32,
    pub item_id: i32,
    pub amount: i32,
}
