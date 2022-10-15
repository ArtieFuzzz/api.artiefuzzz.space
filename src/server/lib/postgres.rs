use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{Connection, PgConnection};
use diesel::result::Error;
use std::env::var;

use crate::models::CreateLogin;

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBManager {
    connection: PooledPg,
}

impl DBManager {
    pub fn new(connection: PooledPg) -> Self {
        DBManager { connection }
    }

    pub fn create_user(&self, user: CreateLogin) -> Result<CreateLogin, Error> {
        // use crate::models::*;
        use crate::schema::logins;

        diesel::insert_into(logins::table)
            .values(&user)
            .get_result(&mut self.connection) // Requires fixing
    }
}

pub fn pq_connect() -> PgConnection {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set...");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn validate_user(conn: &mut PgConnection, usr: String) -> String {}

pub fn create_user(conn: &mut PgConnection, usr: String, pass: String) -> bool {}
