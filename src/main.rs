pub use self::error::{Error, Result};

use auth::KEY;
use axum::{
    http::HeaderValue, middleware, response::{Html, IntoResponse}, Router
};
use std::{fs, net::SocketAddr, path::PathBuf};
use tower_cookies::{Key, CookieManagerLayer};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};

mod api;
mod auth;
mod error;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API router and add middleware to require auth
    let routes_apis = api::routes();
    //.route_layer(middleware::from_fn(auth::middleware::mw_require_auth));

    // Generate cryptographic key for cookies
    let _ = KEY.set(Key::try_generate().unwrap_or(Key::from(b"THISISANUNSAFEKEY_7m893Peh3dFnNhk0o1bOXPHbG7J88GIxiei4x35nkGr5HPr/+sEFMMHI9jw3ehL4ERaRAtrXLN+thqRXmEz+Lw")));

    // Combine all routers
	let mut routes_all = Router::new()
        .nest("/auth", auth::routes::routes())
        .nest("/api", api::routes().route_layer(middleware::from_fn(auth::middleware::mw_require_auth))) // Get API router and add middleware to require auth
        .nest_service("/assets", ServeDir::new("./frontend/dist/assets")) // Serve static files for frontend
        .layer(middleware::from_fn(auth::middleware::mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback(serve_webpage);

    // Permit CORS from dev server if not running in production mode
    if cfg!(debug_assertions) {
        println!("Running in dev mode, allowing CORS requests from React dev server at localhost:5173");
        let cors_layer = CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_headers(Any);
        routes_all = routes_all.layer(cors_layer);
    }

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

// Serve index.html as a fallback for all other routes, since react router will handle frontend routing paths
async fn serve_webpage() -> impl IntoResponse {
    // Read index.html file and return it as HTML, otherwise return a simple error page
    // Will return error page if the frontend hasn't been built! (see README for instructions)
    let index_path = PathBuf::from("./frontend/dist/index.html");
    match fs::read_to_string(index_path) {
        Ok(index_content) => Html(index_content),
        Err(err) => Html(format!("<h1>Error loading homepage</h1><p>{}</p>", err.to_string()).to_owned()),
    }
}
