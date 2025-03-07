use axum::{http::StatusCode, middleware, response::IntoResponse, Router};

use crate::auth::middleware::require_admin;

mod account;
mod admin;
mod api;
mod cart;
mod reviews;

pub fn router() -> Router {
    Router::new()
        .merge(api::routes())
        .merge(reviews::routes())
        .merge(cart::routes())
        .merge(account::routes())
        .nest(
            "/admin",
            admin::router().route_layer(middleware::from_fn(require_admin)),
        )
        .fallback(api_404)
}

// If a call is made to a non-existant API route, return a 404
async fn api_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not a valid API path")
}
