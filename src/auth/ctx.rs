use axum::{
	extract::FromRequestParts,
	http::request::Parts,
};

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i32,
	username: String,
	firstname: String,
	surname: String,
	role: String,
}

// Constructor.
impl Ctx {
	pub fn new(user_id: i32, username: String, firstname: String, surname: String, role: String) -> Self {
		Self { user_id, username, firstname, surname, role }
	}
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

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i32 {
		self.user_id
	}

	pub fn username(&self) -> String {
		self.username.clone()
	}

	pub fn first_name(&self) -> String {
		self.firstname.clone()
	}

	pub fn surname(&self) -> String {
		self.surname.clone()
	}

	pub fn is_admin(&self) -> bool {
		self.role == "admin"
	}
}
