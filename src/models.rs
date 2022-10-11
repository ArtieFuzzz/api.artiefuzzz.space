use diesel::prelude::*;

#[derive(Queryable)]
pub struct Login {
  pub username: String,
  pub pwd: String
}