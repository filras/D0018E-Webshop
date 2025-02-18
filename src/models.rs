use crate::schema::*;
use diesel::prelude::*;
use diesel::Queryable;
use serde::Deserialize;
#[derive(Queryable, Insertable, Identifiable, Selectable, serde::Serialize)]
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

#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub price: i32,
    pub in_stock: i32,
    pub average_rating: Option<f32>,
    pub discounted_price: Option<i32>,
}

#[derive(Queryable, Selectable)]
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
