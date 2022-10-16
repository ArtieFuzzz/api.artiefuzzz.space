use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env::var;
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub usr: String,
    pub exp: usize,
}

/// Attempts to sign a token, failing results in a panic
pub fn sign_token(usr: String) -> String {
    let expiration = Utc::now() + Duration::days(7);
    let usr_claims = Claims {
        usr: usr.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    let secret = var("JWT_SECRET").unwrap();
    let key = &EncodingKey::from_base64_secret(secret.as_ref()).unwrap();
    let jwt_token = encode(&Header::new(Algorithm::HS512), &usr_claims, key).unwrap();
    return jwt_token;
}

pub fn validate_token(token: &str) -> bool {
    let secret = var("JWT_SECRET").unwrap();
    let key = &DecodingKey::from_base64_secret(secret.as_ref()).unwrap();

    let tkn_claims = decode::<Claims>(token, key, &Validation::new(Algorithm::HS512)).unwrap();
    let current_time = Utc::now().timestamp() as usize;
    // Validate expiration time.
    if tkn_claims.claims.exp < current_time
    {
        return false;
    }

    return true;
}
