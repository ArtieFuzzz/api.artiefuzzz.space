use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub usr: String,
    pub exp: usize,
}

const JWT_ALGORITIHM: Algorithm = Algorithm::HS512;

/// Attempts to sign a token, failing results in a panic
pub fn sign_token(usr: String) -> String {
    let usr_claims = Claims {
        usr: usr.to_owned(),
        exp: 0, // TODO: Make keys expire in a week (7 Days)
    };

    let secret = var("JWT_SECRET").unwrap();
    let key = &EncodingKey::from_base64_secret(secret.as_ref()).unwrap();
    let jwt_token = encode(&Header::new(JWT_ALGORITIHM), &usr_claims, key).unwrap();

    return jwt_token;
}

pub fn validate_token(token: &str) -> bool {
    let secret = var("JWT_SECRET").unwrap();
    let key = &DecodingKey::from_base64_secret(secret.as_ref()).unwrap();

    let tkn_claims = decode::<Claims>(token, key, &Validation::new(JWT_ALGORITIHM)).unwrap();

    // Validate expiration time.
    if tkn_claims.claims.exp < 0 /* Place Holder */
    {
        return false;
    }

    return true;
}
