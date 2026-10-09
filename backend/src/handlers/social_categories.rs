use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    config::AppState,
    models::social_category::{CreateSocialCategory, SocialCategory, UpdateSocialCategory},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        validation,
    },
};

/// List the categories of an account
#[utoipa::path(
    get,
    path = "/accounts/{account_id}/categories",
    tag = "Social Categories",
    security(("bearer" = [])),
    params(
        ("account_id" = Uuid, Path, description = "Account to list categories of")
    ),
    responses(
        (status = 200, description = "Categories of the account", body = Vec<SocialCategory>),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn list_social_categories(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<SocialCategory>>, AppError> {
    let categories = SocialCategory::list(&state.db, user_id, account_id).await?;
    Ok(Json(categories))
}

/// Create a category under an account
#[utoipa::path(
    post,
    path = "/accounts/{account_id}/categories",
    tag = "Social Categories",
    security(("bearer" = [])),
    params(
        ("account_id" = Uuid, Path, description = "Account the category belongs to")
    ),
    request_body = CreateSocialCategory,
    responses(
        (status = 201, description = "Category created", body = SocialCategory),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Account not found", body = ErrorBody)
    )
)]
pub async fn create_social_category(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(account_id): Path<Uuid>,
    Json(input): Json<CreateSocialCategory>,
) -> Result<(StatusCode, Json<SocialCategory>), AppError> {
    let name = validation::required_name(&input.name)?;
    let icon = validation::clean_optional(input.icon);

    let category = SocialCategory::create(&state.db, user_id, account_id, &name, icon.as_deref())
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(category)))
}

/// Rename a category, change its icon, or reorder it
#[utoipa::path(
    patch,
    path = "/social-categories/{category_id}",
    tag = "Social Categories",
    security(("bearer" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category to update")
    ),
    request_body = UpdateSocialCategory,
    responses(
        (status = 200, description = "Category updated", body = SocialCategory),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Category not found", body = ErrorBody)
    )
)]
pub async fn update_social_category(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
    Json(input): Json<UpdateSocialCategory>,
) -> Result<Json<SocialCategory>, AppError> {
    let input = UpdateSocialCategory {
        name: input
            .name
            .as_deref()
            .map(validation::required_name)
            .transpose()?,
        // An empty string is kept on purpose: it tells the database to clear the icon
        icon: input.icon.map(|i| i.trim().to_string()),
        position: input.position,
    };

    let category = SocialCategory::update(&state.db, user_id, category_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(category))
}

/// Delete a category together with its links
#[utoipa::path(
    delete,
    path = "/social-categories/{category_id}",
    tag = "Social Categories",
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
pub async fn delete_social_category(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if SocialCategory::delete(&state.db, user_id, category_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
