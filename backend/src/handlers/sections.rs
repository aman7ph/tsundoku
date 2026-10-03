use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    config::AppState,
    models::section::{CreateSection, Section, UpdateSection},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        validation,
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
    let name = validation::required_name(&input.name)?;
    let icon = validation::clean_optional(input.icon);

    let section = Section::create(&state.db, user_id, &name, icon.as_deref()).await?;
    Ok((StatusCode::CREATED, Json(section)))
}

/// Rename a section, change its icon, or reorder it
#[utoipa::path(
    patch,
    path = "/sections/{section_id}",
    tag = "Sections",
    security(("dev_user" = [])),
    params(
        ("section_id" = Uuid, Path, description = "Section to update")
    ),
    request_body = UpdateSection,
    responses(
        (status = 200, description = "Section updated", body = Section),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Section not found", body = ErrorBody)
    )
)]
pub async fn update_section(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(section_id): Path<Uuid>,
    Json(input): Json<UpdateSection>,
) -> Result<Json<Section>, AppError> {
    let input = UpdateSection {
        name: input
            .name
            .as_deref()
            .map(validation::required_name)
            .transpose()?,
        // An empty string is kept on purpose: it tells the database to clear the icon
        icon: input.icon.map(|i| i.trim().to_string()),
        position: input.position,
    };

    let section = Section::update(&state.db, user_id, section_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(section))
}

/// Delete a section together with all its categories and resources
#[utoipa::path(
    delete,
    path = "/sections/{section_id}",
    tag = "Sections",
    security(("dev_user" = [])),
    params(
        ("section_id" = Uuid, Path, description = "Section to delete")
    ),
    responses(
        (status = 204, description = "Section deleted"),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Section not found", body = ErrorBody)
    )
)]
pub async fn delete_section(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(section_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if Section::delete(&state.db, user_id, section_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
