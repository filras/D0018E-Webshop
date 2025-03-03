use axum::{http::StatusCode, middleware, response::IntoResponse, Router};

use crate::auth::middleware::require_auth;
mod account;
mod api;
mod cart;

pub fn router() -> Router {
    Router::new()
        .merge(api::routes().route_layer(middleware::from_fn(require_auth)))
        .merge(account::router())
        .fallback(api_404)
}

// If a call is made to a non-existant API route, return a 404
async fn api_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not a valid API path")
}
