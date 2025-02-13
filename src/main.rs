pub use self::error::{Error, Result};

use crate::ctx::Ctx;
use crate::model::ModelController;
use auth::KEY;
use axum::{
    middleware, Json, Router,
    http::{Method, Uri},
    response::{IntoResponse, Response},
    routing::get_service,
};
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::{Key, CookieManagerLayer};
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod model;
mod auth;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize ModelController.
	let mc = ModelController::new().await?;

	let routes_apis = api::routes()
		.route_layer(middleware::from_fn(auth::middleware::mw_require_auth));

    // Generate cryptographic key for cookies
    let _ = KEY.set(Key::try_generate().unwrap_or(Key::from(b"THISISANUNSAFEKEY_7m893Peh3dFnNhk0o1bOXPHbG7J88GIxiei4x35nkGr5HPr/+sEFMMHI9jw3ehL4ERaRAtrXLN+thqRXmEz+Lw")));

	let routes_all = Router::new()
		// .merge(routes_hello())
		.merge(auth::routes::routes())
		.nest("/api", routes_apis)
		// .layer(middleware::map_response(main_response_mapper))
		.layer(middleware::from_fn_with_state(
			mc.clone(),
			auth::middleware::mw_ctx_resolver,
		))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static());

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);

	axum::serve(listener, routes_all.into_make_service())
    .await
    .unwrap();

Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/static", get_service(ServeDir::new("./frontend/dist")))
}

async fn main_response_mapper(
	ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

				println!("    ->> client_error_body: {client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	// TODO: Need to hander if log_request fail (but should not fail request)
	// let _ =
	// 	log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}
