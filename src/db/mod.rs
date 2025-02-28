use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;

pub fn connect_to_db() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
