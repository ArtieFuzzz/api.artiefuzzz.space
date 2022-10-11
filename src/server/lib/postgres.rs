use std::env::var;
use diesel::{PgConnection, Connection};

pub fn pq_connect() -> PgConnection  {
  let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set...");

  PgConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}