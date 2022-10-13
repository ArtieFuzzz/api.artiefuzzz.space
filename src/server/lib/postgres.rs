// use crate::models::*;
// use crate::schema::logins::dsl::*;
// use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use std::env::var;

pub fn pq_connect() -> PgConnection {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set...");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
