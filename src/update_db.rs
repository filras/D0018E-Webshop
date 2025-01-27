use mysql::{prelude::*, *};
use dotenv;

#[ignore = "non_snake_case"]
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  
  // Verify presence of .env with all required variables
  dotenv::dotenv().map_err(|_| String::from(".env not found"))?;
  let MYSQL_USER = dotenv::var("MYSQL_USER").map_err(|e| e.to_string() + ": MYSQL_USER")?;
  let MYSQL_PASSWORD = dotenv::var("MYSQL_PASSWORD").map_err(|e| e.to_string() + ": MYSQL_PASSWORD")?;
  let MYSQL_HOST = dotenv::var("MYSQL_HOST").map_err(|e| e.to_string() + ": MYSQL_HOST")?;
  let MYSQL_PORT = dotenv::var("MYSQL_PORT").map_err(|e| e.to_string() + ": MYSQL_PORT")?;
  let MYSQL_DB_NAME = dotenv::var("MYSQL_DB_NAME").map_err(|e| e.to_string() + ": MYSQL_DB_NAME")?;
  println!("Successfully read .env");

  // Start connection to MYSQL
  // let url = format!("mysql://{}:{}@{}:{}/{}", MYSQL_USER, MYSQL_PASSWORD, MYSQL_HOST, MYSQL_PORT, MYSQL_DB_NAME).to_string();
  let opts = OptsBuilder::new()
    .user(Some(MYSQL_USER))
    .pass(Some(MYSQL_PASSWORD))
    .ip_or_hostname(Some(MYSQL_HOST))
    .tcp_port(MYSQL_PORT.parse::<u16>().unwrap());
    // .db_name(Some(MYSQL_DB_NAME));

  let pool = Pool::new(opts)?;
  let mut conn = pool.get_conn()?;

  // Create and enter database
  conn.query_drop(format!("CREATE DATABASE IF NOT EXISTS {};", MYSQL_DB_NAME))?;
  conn.query_drop(format!("USE {};", MYSQL_DB_NAME))?;

  // Create user table
  conn.query_drop(format!(
    "CREATE TABLE IF NOT EXISTS users (
      id int not null,
      username text not null,
      password_hash text not null,
      firstname text not null,
      surname text not null,
      email text not null,
      role text not null,
      address text,
      zipcode text,
      co text,
      country text
    );",
    ))?;


  conn.query_iter(
    r"DESCRIBE users;"
  )?
  .for_each(|row| println!("{:?}", row));

  Ok(())
}