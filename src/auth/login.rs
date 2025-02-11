use axum::Json;
use axum_session_auth::*;
use serde::Deserialize;

use super::{NullPool, User};

struct TestUser {
  pub username: &'static str,
  pub password: &'static str,
  pub id: i64,
}

const USERS: &'static [TestUser] = &[
  TestUser{
      id: 123,
      username: "Olle",
      password: "sus",
  },
  TestUser{
      id: 456,
      username: "admin",
      password: "admin",
  },
  TestUser{
      id: 789,
      username: "Viggo",
      password: "test",
  }
];

#[derive(Deserialize)]
pub struct Login {
  username: String,
  password: String,
}

pub async fn handle_login(
  auth: AuthSession<User, i64, SessionNullPool, NullPool>,
  login: Json<Login>,
) -> String {
  // TEMP: Search through test users to find the one with the right user/pass combo
  for test_user in USERS {
    if login.username == test_user.username && login.password == test_user.password {
      auth.login_user(test_user.id);
    }
  }

  "Logged in".to_owned()
}

pub async fn handle_logout(
  auth: AuthSession<User, i64, SessionNullPool, NullPool>
) -> String {
  auth.logout_user();
  "Logged out".to_owned()
}
