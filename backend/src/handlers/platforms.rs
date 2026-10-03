use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    config::AppState,
    models::platform::{CreatePlatform, Platform, UpdatePlatform},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        validation,
    },
};

/// List the user's platforms
#[utoipa::path(
    get,
    path = "/platforms",
    tag = "Platforms",
    security(("dev_user" = [])),
    responses(
        (status = 200, description = "The user's platforms", body = Vec<Platform>),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn list_platforms(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
) -> Result<Json<Vec<Platform>>, AppError> {
    let platforms = Platform::list(&state.db, user_id).await?;
    Ok(Json(platforms))
}

/// Create a platform
#[utoipa::path(
    post,
    path = "/platforms",
    tag = "Platforms",
    security(("dev_user" = [])),
    request_body = CreatePlatform,
    responses(
        (status = 201, description = "Platform created", body = Platform),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 409, description = "A platform with this key already exists", body = ErrorBody)
    )
)]
pub async fn create_platform(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Json(input): Json<CreatePlatform>,
) -> Result<(StatusCode, Json<Platform>), AppError> {
    let name = validation::required_name(&input.name)?;
    let key = validation::clean_optional(input.key)
        .as_deref()
        .map(validation::platform_key)
        .transpose()?;
    let icon = validation::clean_optional(input.icon);

    let platform =
        Platform::create(&state.db, user_id, key.as_deref(), &name, icon.as_deref()).await?;

    Ok((StatusCode::CREATED, Json(platform)))
}

/// Rename a platform, change its icon, or reorder it
#[utoipa::path(
    patch,
    path = "/platforms/{platform_id}",
    tag = "Platforms",
    security(("dev_user" = [])),
    params(
        ("platform_id" = Uuid, Path, description = "Platform to update")
    ),
    request_body = UpdatePlatform,
    responses(
        (status = 200, description = "Platform updated", body = Platform),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Platform not found", body = ErrorBody)
    )
)]
pub async fn update_platform(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(platform_id): Path<Uuid>,
    Json(input): Json<UpdatePlatform>,
) -> Result<Json<Platform>, AppError> {
    let input = UpdatePlatform {
        name: input
            .name
            .as_deref()
            .map(validation::required_name)
            .transpose()?,
        // An empty string is kept on purpose: it tells the database to clear the icon
        icon: input.icon.map(|i| i.trim().to_string()),
        position: input.position,
    };

    let platform = Platform::update(&state.db, user_id, platform_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(platform))
}

/// Delete a platform together with its accounts
#[utoipa::path(
    delete,
    path = "/platforms/{platform_id}",
    tag = "Platforms",
    security(("dev_user" = [])),
    params(
        ("platform_id" = Uuid, Path, description = "Platform to delete")
    ),
    responses(
        (status = 204, description = "Platform deleted"),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Platform not found", body = ErrorBody)
    )
)]
pub async fn delete_platform(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(platform_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if Platform::delete(&state.db, user_id, platform_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
