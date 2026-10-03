use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    config::AppState,
    models::category::{Category, CreateCategory},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
    },
};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListParams {
    /// List the children of this category. Omit for the top level of the section.
    pub parent_id: Option<Uuid>,
}

/// List the categories of a section, one level at a time
#[utoipa::path(
    get,
    path = "/sections/{section_id}/categories",
    tag = "Categories",
    security(("dev_user" = [])),
    params(
        ("section_id" = Uuid, Path, description = "Section to list categories of"),
        ListParams
    ),
    responses(
        (status = 200, description = "Categories at the requested level", body = Vec<Category>),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn list_categories(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(section_id): Path<Uuid>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = Category::list(&state.db, user_id, section_id, params.parent_id).await?;
    Ok(Json(categories))
}

/// Create a category or, with `parent_id`, a subcategory
#[utoipa::path(
    post,
    path = "/sections/{section_id}/categories",
    tag = "Categories",
    security(("dev_user" = [])),
    params(
        ("section_id" = Uuid, Path, description = "Section the category belongs to")
    ),
    request_body = CreateCategory,
    responses(
        (status = 201, description = "Category created", body = Category),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Section or parent category not found", body = ErrorBody)
    )
)]
pub async fn create_category(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(section_id): Path<Uuid>,
    Json(input): Json<CreateCategory>,
) -> Result<(StatusCode, Json<Category>), AppError> {
    let name = input.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest("name cannot be empty".into()));
    }

    let category = Category::create(
        &state.db,
        user_id,
        section_id,
        input.parent_id,
        name,
        input.icon.as_deref(),
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(category)))
}
