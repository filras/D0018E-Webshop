use axum::{http::StatusCode, middleware, response::IntoResponse, Router};

use crate::auth::middleware::{require_admin, require_auth};

mod account;
mod admin;
mod api;
mod cart;
mod order;

pub fn router() -> Router {
    Router::new()
        .merge(api::routes().route_layer(middleware::from_fn(require_auth)))
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
