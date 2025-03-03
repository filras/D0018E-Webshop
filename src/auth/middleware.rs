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
	http::{Request, StatusCode},
	middleware::Next,
	response::IntoResponse,
};
use diesel::{query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl, SelectableHelper};
use tower_cookies::{Cookie, Cookies};

use super::{session::update_user_session, COOKIE_EXPIRATION_TIME_SECONDS};

// Middleware to force ctx (auth) for all paths under a router
// If this middleware is used on a router or path, ctx can safely be unwrapped to retrieve user data
pub async fn require_auth(
	ctx: Result<Ctx, String>,
	req: Request<Body>,
	next: Next,
) -> impl IntoResponse {
	match ctx {
		Ok(_) => Ok(next.run(req).await),
		Err(_) => Err((StatusCode::UNAUTHORIZED, "You need to be logged in to access this endpoint").into_response()),
	}.into_response()
}

// Middleware to force ctx (auth) for all paths under a router AND only allow users with admin privileges
// If this middleware is used on a router or path, ctx can safely be unwrapped to retrieve user data
pub async fn require_admin(
	ctx: Result<Ctx, String>,
	req: Request<Body>,
	next: Next,
) -> impl IntoResponse {
	match ctx {
		Ok(user) => {
			// User is signed in, so check if they are admin
			match user.is_admin() {
				true => Ok(next.run(req).await),
				false => Err((StatusCode::FORBIDDEN, "You must be an admin user to access this endpoint").into_response())
			}
		},
		// Otherwise, send unauthorized
		Err(_) => Err((StatusCode::UNAUTHORIZED, "You need to be logged in to access this endpoint").into_response()),
	}
}

#[derive(PartialEq)]
enum TokenError {
		NoToken,
		TokenExpired,
		TokenMalformed,
}

// Middleware to perform context (Ctx) resolution from cookies
pub async fn ctx_resolver(
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> impl IntoResponse {
	// Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());

	// Compute cookie value
	let token_result = private_cookies.get(COOKIE_NAME)
		.ok_or(TokenError::NoToken)
		.and_then(parse_token);
	
	match token_result {
		// Remove the cookie if something went wrong
		Err(e) => {
			private_cookies.remove(Cookie::build(COOKIE_NAME).path("/").into());

			// Return error if token has expired
			if e == TokenError::TokenExpired {
				return (StatusCode::UNAUTHORIZED, "Token expired").into_response()
			}
		},

		// Get user data from DB based on user id in token
		Ok(user_id) => {
			let conn = &mut connect_to_db();
			let result = users.filter(id.eq(user_id))
					.select(User::as_select())
					.first::<User>(conn);
			if result.is_err() {
				// User might have been deleted, so remove their session, then return an error
				private_cookies.remove(Cookie::build(COOKIE_NAME).path("/").into());
				return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to fetch user data").into_response()
			}
			let user = result.unwrap();
		
			// Store the ctx as a request extension
			let context_data = Ctx::new(
				user_id,
				user.username.to_owned(),
				user.firstname.to_owned(),
				user.surname.to_owned(),
				user.role.to_owned(),
			);
			req.extensions_mut().insert(context_data);

			// Update token with a new timestamp to prevent expiry while still active
			update_user_session(cookies, user_id);
		}
	}

	// If resolve was successful, continue request
	(next.run(req).await).into_response()
}

// Parse userid and timestamp from token
fn parse_token(token: Cookie) -> Result<i32, TokenError> {
	// Token is built like this: "userid:timestamp"
	// We split the token string by ":", parse and destructure into uid and timestamp as Result<i64, ParseIntError>
	let split_token = token.value().split(":").take(2).collect::<Vec<&str>>();
	let [uid, timestamp] = <[&str; 2]>::try_from(split_token).ok()
		.unwrap_or(["", ""])
		.map(|str| str.parse::<i64>());

	// Check if token was properly parsed
	if uid.is_err() || timestamp.is_err() {
		return Err(TokenError::TokenMalformed)
	}

	// Check if token has expired
	let current_time_utc = chrono::Utc::now().timestamp();
	if timestamp.unwrap() + COOKIE_EXPIRATION_TIME_SECONDS < current_time_utc {
		return Err(TokenError::TokenExpired)
	}

	Ok(uid.unwrap() as i32)
}
