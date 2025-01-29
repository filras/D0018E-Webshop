mod file_handler;
mod api;
mod types;
use types::Person;
use api::get_items;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use file_handler::static_router;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};



#[tokio::main]
async fn main() {
    // let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/items{skip}", get(get_items));
        // .route("/people", get(get_people))
        // .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];

    (StatusCode::OK, Json(people))
}
