use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = create::schema::items, Selectable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub price: i32,
    pub in_stock: i32,
    pub average_rating: f32,
    pub discounted_price: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = create::schema::items, Selectable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub firstname: String,
    pub surname: String,
    pub email: String,
    pub role: String,
    pub address: String,
    pub zipcode: String,
    pub co: String,
    pub country: String,
}
