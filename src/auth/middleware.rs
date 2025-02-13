use crate::ctx::Ctx;
use crate::auth::{AUTH_TOKEN, COOKIE_NAME, KEY};
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

pub async fn mw_require_auth(
	ctx: Result<Ctx>,
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
	println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response> {
	println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

	let key = KEY.get();
	let private_cookies = cookies.private(key.unwrap());

	if private_cookies.get(COOKIE_NAME).is_some() {
			println!("yay we have a cookie!");
	}

	// Compute cookie value
	let result_ctx = match private_cookies.get(COOKIE_NAME)
		.ok_or(Error::AuthFailNoAuthTokenCookie)
		.and_then(parse_token)
	{
		Ok(user_id) => {
			// TODO: Token components validations.
			Ok(Ctx::new(user_id))
		}
		Err(e) => Err(e),
	};
	
	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
	{
		cookies.remove(Cookie::from(AUTH_TOKEN))
	}

	// Store the ctx_result in the request extension.
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		// println!("->> {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}

// endregion: --- Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: Cookie) -> Result<u64> {
	let uid = token.value().parse::<u64>();

	if uid.is_err() {
		return Err(Error::AuthFailTokenWrongFormat);
	}

	Ok(uid.unwrap())
}
