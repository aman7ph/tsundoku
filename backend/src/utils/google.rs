use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use serde::Deserialize;

use crate::utils::error::AppError;

const GOOGLE_CERTS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

#[derive(Deserialize)]
pub struct GoogleClaims {
    /// Google's permanent id for the person
    pub sub: String,
    pub email: String,
    #[serde(default)]
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
}

fn upstream_error(err: reqwest::Error) -> AppError {
    eprintln!("could not reach Google: {err}");
    AppError::Internal
}

// Fetched on every sign-in. Sign-ins are rare, so no cache yet.
async fn fetch_google_keys(http: &reqwest::Client) -> Result<JwkSet, AppError> {
    http.get(GOOGLE_CERTS_URL)
        .send()
        .await
        .and_then(|response| response.error_for_status())
        .map_err(upstream_error)?
        .json::<JwkSet>()
        .await
        .map_err(upstream_error)
}

/// Checks the signature, expiry, issuer and audience of a Google ID token.
pub async fn verify_id_token(
    http: &reqwest::Client,
    client_id: &str,
    id_token: &str,
) -> Result<GoogleClaims, AppError> {
    // Reject obvious garbage before calling Google
    let header = decode_header(id_token).map_err(|_| AppError::Unauthorized)?;
    let kid = header.kid.ok_or(AppError::Unauthorized)?;

    let keys = fetch_google_keys(http).await?;
    let jwk = keys.find(&kid).ok_or(AppError::Unauthorized)?;
    let key = DecodingKey::from_jwk(jwk).map_err(|_| AppError::Unauthorized)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id]);
    validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]);

    let claims = decode::<GoogleClaims>(id_token, &key, &validation)
        .map_err(|_| AppError::Unauthorized)?
        .claims;

    if !claims.email_verified {
        return Err(AppError::Unauthorized);
    }

    Ok(claims)
}
