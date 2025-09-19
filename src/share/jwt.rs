use crate::config::env::CONFIG;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    errors::Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    // Add other fields as needed
}

pub fn verify_token(token: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&CONFIG.jwt.clone().into_bytes()),
        &Validation::new(Algorithm::HS256),
    )
}

pub fn generate_token(username: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&CONFIG.jwt.clone().into_bytes()),
    )
}
