use crate::{
	auth::{ctx::Ctx, COOKIE_NAME, KEY},
	schema::users::{dsl::users, *},
	db::{
		connect_to_db,
		models::User,
	},
};
use axum::{
	body::Body,
	extract::FromRequestParts,
	http::{request::Parts, Request, StatusCode},
	middleware::Next,
	response::{IntoResponse, Response},
};
use diesel::{query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl, SelectableHelper};
use tower_cookies::{Cookie, Cookies};

// Middleware to force ctx (auth) for all paths under a router
// If this middleware is used on a router or path, ctx can safely be unwrapped to retrieve user data
pub async fn mw_require_auth(
	ctx: Result<Ctx, String>,
	req: Request<Body>,
	next: Next,
) -> impl IntoResponse {
	match ctx {
		Ok(_) => Ok(next.run(req).await),
		Err(_) => Err((StatusCode::UNAUTHORIZED, "You need to be logged in to access this page").into_response()),
		_ => Err("unknown authentication error".into_response())
	}.into_response()
}

// Middleware to perform context (Ctx) resolution from cookies
pub async fn mw_ctx_resolver(
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> impl IntoResponse {
	// Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());

	// Compute cookie value
	let token_result = private_cookies.get(COOKIE_NAME)
		.ok_or("no auth token cookie")
		.and_then(parse_token);
	
	// Remove the cookie if something went wrong
	if token_result.is_err() {
		private_cookies.remove(Cookie::build(COOKIE_NAME).path("/").into());
	} else {
		// Get user data from DB based on user id in token
		let user_id = token_result.unwrap();
		let conn = &mut connect_to_db();
		let result = users.filter(id.eq(user_id))
				.select(User::as_select())
				.load::<User>(conn);
		if result.is_err() {
			return Err::<Response<Body>, String>("Unable to fetch user data".to_string()).into_response()
		}
		let fetched_users = result.unwrap();
		if fetched_users.len() != 1 {
			return Err::<Response<Body>, String>("Unable to fetch user data".to_string()).into_response()
		}
	
		// Store the ctx as a request extension
		let user = fetched_users.first().unwrap();
		let context_data = Ctx::new(
			token_result.unwrap(),
			user.username.to_owned(),
			user.firstname.to_owned(),
			user.surname.to_owned(),
			user.role.to_owned(),
		);
		req.extensions_mut().insert(context_data);
	}

	Ok::<Response<Body>, String>(next.run(req).await).into_response()
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = String;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, String> {
		match parts
			.extensions
			.get::<Ctx>() {
				Some(ctx) => Ok(ctx.clone()),
				None => Err("Missing user ctx".to_string()),
			}
	}
}

// Parse userid from token
fn parse_token(token: Cookie) -> Result<i32, &str> {
	let uid = token.value().parse::<i32>();

	if uid.is_err() {
		return Err("auth token badly formatted");
	}

	Ok(uid.unwrap())
}
