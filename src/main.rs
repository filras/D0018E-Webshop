pub use self::error::{Error, Result};

use auth::KEY;
use axum::{middleware, routing::get_service, Router};
use std::net::SocketAddr;
use tower_cookies::{CookieManagerLayer, Key};
use tower_http::services::ServeDir;

mod api;
mod auth;
mod ctx;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API router and add middleware to require auth
    let routes_apis = api::routes();
    //.route_layer(middleware::from_fn(auth::middleware::mw_require_auth));

    // Generate cryptographic key for cookies
    let _ = KEY.set(Key::try_generate().unwrap_or(Key::from(b"THISISANUNSAFEKEY_7m893Peh3dFnNhk0o1bOXPHbG7J88GIxiei4x35nkGr5HPr/+sEFMMHI9jw3ehL4ERaRAtrXLN+thqRXmEz+Lw")));

    // Combine all routers
    let routes_all = Router::new()
        .merge(auth::routes::routes())
        .nest("/api", routes_apis)
        .layer(middleware::from_fn(auth::middleware::mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // Create TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);

    // Serve all routes on the TCP listener
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Serve static files for frontend
fn routes_static() -> Router {
    Router::new().nest_service("/static", get_service(ServeDir::new("./frontend/dist")))
}
