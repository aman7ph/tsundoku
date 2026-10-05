use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::utils::error::AppError;
use crate::config::TokenLifeSpan;

#[derive(Serialize, Deserialize)]
struct Claims {
    /// The user's id
    sub: Uuid,
    iat: i64,
    exp: i64,
}

pub fn create_access_token(secret: &str, user_id: Uuid) -> Result<String, AppError> {
    let life = TokenLifeSpan::from_env();

    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp(),
        exp: (now + Duration::minutes(life.access_token_life)).timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|err| {
        eprintln!("could not sign access token: {err}");
        AppError::Internal
    })
}

/// Returns the user id when the token is genuine and not expired
pub fn verify_access_token(secret: &str, token: &str) -> Result<Uuid, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims.sub)
    .map_err(|_| AppError::Unauthorized)
}