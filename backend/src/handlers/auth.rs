use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::{
    config::AppState,
    models::{
        session::{GoogleLogin, RefreshRequest, Rotation, Session, TokenPair},
        user::User,
    },
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        google, jwt,
    },
};

fn token_pair(
    state: &AppState,
    user_id: Uuid,
    refresh_token: String,
) -> Result<TokenPair, AppError> {
    Ok(TokenPair {
        access_token: jwt::create_access_token(&state.auth.jwt_secret, user_id)?,
        refresh_token,
        token_type: "Bearer".into(),
        expires_in: jwt::ACCESS_TOKEN_MINUTES * 60,
    })
}

/// Sign in with a Google ID token
#[utoipa::path(
    post,
    path = "/auth/google",
    tag = "Auth",
    request_body = GoogleLogin,
    responses(
        (status = 200, description = "Signed in", body = TokenPair),
        (status = 401, description = "The Google token is invalid, expired or for another app", body = ErrorBody)
    )
)]
pub async fn google_login(
    State(state): State<AppState>,
    Json(input): Json<GoogleLogin>,
) -> Result<Json<TokenPair>, AppError> {
    let claims = google::verify_id_token(
        &state.http,
        &state.auth.google_client_id,
        &input.id_token,
    )
    .await?;

    let user_id = User::upsert_google(
        &state.db,
        &claims.sub,
        &claims.email,
        claims.name.as_deref(),
        claims.picture.as_deref(),
    )
    .await?;

    let refresh_token = Session::start(&state.db, user_id).await?;
    Ok(Json(token_pair(&state, user_id, refresh_token)?))
}

/// Exchange a refresh token for a new token pair
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "New token pair; the old refresh token no longer works", body = TokenPair),
        (status = 401, description = "Unknown, expired, revoked or already used refresh token", body = ErrorBody)
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> Result<Json<TokenPair>, AppError> {
    match Session::rotate(&state.db, &input.refresh_token).await? {
        Rotation::Rotated {
            user_id,
            refresh_token,
        } => Ok(Json(token_pair(&state, user_id, refresh_token)?)),
        Rotation::Rejected => Err(AppError::Unauthorized),
    }
}

/// Sign out: ends the session the refresh token belongs to
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 204, description = "Signed out (also when the token was already unknown)")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> Result<StatusCode, AppError> {
    Session::end(&state.db, &input.refresh_token).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Sign in as the development user. Disabled unless ENABLE_DEV_LOGIN=true.
#[utoipa::path(
    post,
    path = "/auth/dev-login",
    tag = "Auth",
    responses(
        (status = 200, description = "Signed in as the dev user", body = TokenPair),
        (status = 404, description = "Dev login is disabled", body = ErrorBody)
    )
)]
pub async fn dev_login(State(state): State<AppState>) -> Result<Json<TokenPair>, AppError> {
    if !state.auth.dev_login_enabled {
        return Err(AppError::NotFound);
    }

    let user_id = User::upsert_google(
        &state.db,
        "dev",
        "dev@example.com",
        Some("Dev User"),
        None,
    )
    .await?;

    let refresh_token = Session::start(&state.db, user_id).await?;
    Ok(Json(token_pair(&state, user_id, refresh_token)?))
}

/// The signed-in user
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "Auth",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "The current user", body = User),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn me(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
) -> Result<Json<User>, AppError> {
    let user = User::find(&state.db, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(Json(user))
}