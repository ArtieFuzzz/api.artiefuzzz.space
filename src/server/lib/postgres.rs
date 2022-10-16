use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::{Connection, PgConnection};
use std::env::var;

use crate::models::{CreateLogin, Login};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBManager {
    connection: PooledPg,
}

impl DBManager {
    pub fn new(connection: PooledPg) -> DBManager {
        DBManager { connection }
    }

    pub fn create_user(&mut self, user: CreateLogin) -> Result<Login, Error> {
        use crate::schema::logins;

        diesel::insert_into::<logins::table>(logins::table)
            .values(&user)
            .get_result(&mut self.connection)
    }

    pub fn fetch_user(&mut self, usrname: String) -> Result<Vec<Login>, Error> {
        use crate::schema::logins::dsl::*;

        logins
            .filter(username.eq(usrname))
            .load(&mut self.connection)
    }
}

pub fn pq_connect() -> PgConnection {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set...");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
