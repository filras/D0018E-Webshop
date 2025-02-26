#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i32,
	username: String,
	firstname: String,
	surname: String,
}

// Constructor.
impl Ctx {
	pub fn new(user_id: i32, username: String, firstname: String, surname: String,) -> Self {
		Self { user_id, username, firstname, surname }
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
}
