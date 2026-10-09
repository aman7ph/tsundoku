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
    models::category::{Category, CreateCategory, MoveCategory, UpdateCategory},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        validation,
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
    security(("bearer" = [])),
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
    security(("bearer" = [])),
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
    let name = validation::required_name(&input.name)?;
    let icon = validation::clean_optional(input.icon);

    let category = Category::create(
        &state.db,
        user_id,
        section_id,
        input.parent_id,
        &name,
        icon.as_deref(),
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(category)))
}

/// Rename a category, change its icon, or reorder it
#[utoipa::path(
    patch,
    path = "/categories/{category_id}",
    tag = "Categories",
    security(("bearer" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category to update")
    ),
    request_body = UpdateCategory,
    responses(
        (status = 200, description = "Category updated", body = Category),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Category not found", body = ErrorBody)
    )
)]
pub async fn update_category(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
    Json(input): Json<UpdateCategory>,
) -> Result<Json<Category>, AppError> {
    let input = UpdateCategory {
        name: input
            .name
            .as_deref()
            .map(validation::required_name)
            .transpose()?,
        // An empty string is kept on purpose: it tells the database to clear the icon
        icon: input.icon.map(|i| i.trim().to_string()),
        position: input.position,
    };

    let category = Category::update(&state.db, user_id, category_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(category))
}

/// Move a category under another parent, or to the top level of its section
#[utoipa::path(
    post,
    path = "/categories/{category_id}/move",
    tag = "Categories",
    security(("bearer" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category to move")
    ),
    request_body = MoveCategory,
    responses(
        (status = 200, description = "Category moved", body = Category),
        (status = 400, description = "Invalid new parent", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Category not found", body = ErrorBody)
    )
)]
pub async fn move_category(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
    Json(input): Json<MoveCategory>,
) -> Result<Json<Category>, AppError> {
    // First tell "category does not exist" (404) apart from "that move is not allowed" (400)
    Category::find(&state.db, user_id, category_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let category = Category::move_to(&state.db, user_id, category_id, input.parent_id)
        .await?
        .ok_or_else(|| {
            AppError::BadRequest(
                "parent must be a category of the same section, and not inside this category"
                    .into(),
            )
        })?;

    Ok(Json(category))
}

/// Delete a category together with its subcategories and resources
#[utoipa::path(
    delete,
    path = "/categories/{category_id}",
    tag = "Categories",
    security(("bearer" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category to delete")
    ),
    responses(
        (status = 204, description = "Category deleted"),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Category not found", body = ErrorBody)
    )
)]
pub async fn delete_category(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if Category::delete(&state.db, user_id, category_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
