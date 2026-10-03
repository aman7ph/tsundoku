use axum::{Json, extract::State, http::StatusCode};

use crate::{
    config::AppState,
    models::section::{CreateSection, Section},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
    },
};

/// List all sections of the current user
#[utoipa::path(
    get,
    path = "/sections",
    tag = "Sections",
    security(("dev_user" = [])),
    responses(
        (status = 200, description = "The user's sections", body = Vec<Section>),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn list_sections(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
) -> Result<Json<Vec<Section>>, AppError> {
    let sections = Section::list(&state.db, user_id).await?;
    Ok(Json(sections))
}

/// Create a new section
#[utoipa::path(
    post,
    path = "/sections",
    tag = "Sections",
    security(("dev_user" = [])),
    request_body = CreateSection,
    responses(
        (status = 201, description = "Section created", body = Section),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn create_section(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Json(input): Json<CreateSection>, // the body extractor must come last
) -> Result<(StatusCode, Json<Section>), AppError> {
    let name = input.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest("name cannot be empty".into()));
    }

    let section = Section::create(&state.db, user_id, name, input.icon.as_deref()).await?;
    Ok((StatusCode::CREATED, Json(section)))
}
