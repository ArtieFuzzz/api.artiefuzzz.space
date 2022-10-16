/* TODO:
 * Actual User Validation
 * 
 * Combines:
 * 
 * ./jwt.rs
 * ../postgres.rs
 * 
 * ------------------
 */

pub fn register_user() {
  // Reduce duplication by checking if the user already exists (This may get handled by the ORM)
}

pub fn login_user() {
  // Check if credentials match the DBs, generate a token with the `sub` as the username, token automatically expire in 24 hours
}

pub fn hash_pwd(pwd: &str) -> String {
  String::from("Placeholder")
}