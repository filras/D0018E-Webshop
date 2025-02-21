#[allow(unused)]
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use diesel::{
    delete, 
    prelude::*,
    dsl::insert_into,
};
use serde::Deserialize;

use crate::db::{
    connect_to_db,
    models::{Item, NewItem, User, NewUser},
    schema::{
        items::{dsl::items, *},
        users::{dsl::users, *},
    }
};

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

#[derive(Debug, Deserialize)]
struct Uname {
    username: String,
}

// routes for api

pub fn routes() -> Router {
    Router::new()
        .route("/items", get(get_items).post(post_items))
        .route(
            "/users",
            get(get_user)
                .post(post_user)
                .delete(delete_user)
                .put(update_user),
        )
        .route("/test", get(|| async {"Hello, world!".into_response()}))
}

async fn update_user(uname: Query<Uname>, data: Json<NewUser>) -> impl IntoResponse {
    let uname: Uname = uname.0;
    let rcv_user: NewUser = data.0;
    use crate::db::schema::users::dsl::*;
    let conn = &mut connect_to_db();
    let values = (
        username.eq(rcv_user.username),
        password_hash.eq(rcv_user.password_hash),
        firstname.eq(rcv_user.firstname),
        surname.eq(rcv_user.surname),
        email.eq(rcv_user.email),
        role.eq(rcv_user.role),
        address.eq(rcv_user.address),
        zipcode.eq(rcv_user.zipcode),
        co.eq(rcv_user.co),
        country.eq(rcv_user.country),
    );

    return match diesel::update(users)
        .filter(username.eq(uname.username))
        .set(values)
        .execute(conn) {
        Ok(_) => (StatusCode::OK, "User updated").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
    
}

async fn delete_user(uname: Query<Uname>) -> impl IntoResponse {
    let uname: Uname = uname.0;
    let conn = &mut connect_to_db();
    let old_count = users.count().first::<i64>(conn);

    let result = delete(users.filter(username.eq(uname.username)))
        .execute(conn);
    assert_eq!(old_count.map(|count| count - 1), users.count().first(conn));
    return match result {
        Ok(_) => (StatusCode::OK, "User deleted").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

async fn get_user(uname: Query<Uname>) -> impl IntoResponse {
    let uname: Uname = uname.0;
    let conn = &mut connect_to_db();
    return match users
        .filter(username.eq(uname.username))
        .select(User::as_select())
        .load::<User>(conn) {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

async fn post_user(data: Json<NewUser>) -> impl IntoResponse {
    let rcv_user: NewUser = data.0;
    let conn = &mut connect_to_db();
    let values = (
        username.eq(rcv_user.username),
        password_hash.eq(rcv_user.password_hash),
        firstname.eq(rcv_user.firstname),
        surname.eq(rcv_user.surname),
        email.eq(rcv_user.email),
        role.eq(rcv_user.role),
        address.eq(rcv_user.address),
        zipcode.eq(rcv_user.zipcode),
        co.eq(rcv_user.co),
        country.eq(rcv_user.country),
    );

    return match insert_into(users)
        .values(values)
        .execute(conn) {
        Ok(_) => (StatusCode::OK, "User recieved").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

async fn get_items(pagination: Query<Pagination>) -> impl IntoResponse {
    let pagination: Pagination = pagination.0;
    let conn = &mut connect_to_db();

    return match items
        .offset(((pagination.page - 1) * pagination.per_page) as i64)
        .limit(pagination.per_page as i64)
        .select(Item::as_select())
        .load::<Item>(conn) {
            Ok(results) => (StatusCode::OK, Json(results)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
}

async fn post_items(data: Json<NewItem>) -> impl IntoResponse {
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
