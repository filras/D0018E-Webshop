use crate::ctx::Ctx;
use crate::auth::{COOKIE_NAME, KEY};
use crate::{Error, Result};
use axum::{
	body::Body,
	extract::FromRequestParts,
	http::request::Parts,
	http::Request,
	middleware::Next,
	response::Response,
};
use tower_cookies::{Cookie, Cookies};

// Middleware to force ctx (auth) for all paths under a router
pub async fn mw_require_auth(
	ctx: Result<Ctx>,
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
	ctx?;
	Ok(next.run(req).await)
}

// Middleware to perform context (Ctx) resolution from cookies
pub async fn mw_ctx_resolver(
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response> {
	// Create private cookie jar from global static KEY to validate cookies
	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());

	// Compute cookie value
	let result_ctx = match private_cookies.get(COOKIE_NAME)
		.ok_or(Error::AuthFailNoAuthTokenCookie)
		.and_then(parse_token)
	{
		Ok(user_id) => Ok(Ctx::new(user_id)),
		Err(e) => Err(e),
	};
	
	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
	{
		private_cookies.remove(Cookie::build(COOKIE_NAME).path("/").into());
	}

	// Store the ctx as a request extension
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {

		parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}

// Parse userid from token
fn parse_token(token: Cookie) -> Result<u64> {
	let uid = token.value().parse::<u64>();

	if uid.is_err() {
		return Err(Error::AuthFailTokenWrongFormat);
	}

	Ok(uid.unwrap())
}
