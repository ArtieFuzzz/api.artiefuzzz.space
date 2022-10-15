use diesel::prelude::*;
use super::schema::logins;

#[derive(Queryable)]
pub struct Login {
  pub username: String,
  pub pwd: String
}

#[derive(Insertable)]
#[table_name="logins"]
pub struct CreateLogin {
  pub pwd: String
}