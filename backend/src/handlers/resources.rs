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
    models::resource::{CreateResource, Resource, UpdateResource},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        validation,
    },
};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListResourcesParams {
    /// Only resources with this visited value
    pub visited: Option<bool>,
    /// Only resources with this shareable value
    pub shareable: Option<bool>,
}

/// List the resources of a category, newest first
#[utoipa::path(
    get,
    path = "/categories/{category_id}/resources",
    tag = "Resources",
    security(("dev_user" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category to list resources of"),
        ListResourcesParams
    ),
    responses(
        (status = 200, description = "Resources in the category", body = Vec<Resource>),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn list_resources(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
    Query(params): Query<ListResourcesParams>,
) -> Result<Json<Vec<Resource>>, AppError> {
    let resources = Resource::list(
        &state.db,
        user_id,
        category_id,
        params.visited,
        params.shareable,
    )
    .await?;

    Ok(Json(resources))
}

/// Save a link into a category
#[utoipa::path(
    post,
    path = "/categories/{category_id}/resources",
    tag = "Resources",
    security(("dev_user" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category the resource is saved in")
    ),
    request_body = CreateResource,
    responses(
        (status = 201, description = "Resource saved", body = Resource),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Category not found", body = ErrorBody)
    )
)]
pub async fn create_resource(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
    Json(input): Json<CreateResource>,
) -> Result<(StatusCode, Json<Resource>), AppError> {
    let input = CreateResource {
        url: validation::http_url(&input.url)?,
        title: validation::clean_optional(input.title),
        description: validation::clean_optional(input.description),
        visited: input.visited,
        shareable: input.shareable,
    };

    let resource = Resource::create(&state.db, user_id, category_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(resource)))
}

/// Edit a resource, or toggle visited / shareable
#[utoipa::path(
    patch,
    path = "/resources/{resource_id}",
    tag = "Resources",
    security(("dev_user" = [])),
    params(
        ("resource_id" = Uuid, Path, description = "Resource to update")
    ),
    request_body = UpdateResource,
    responses(
        (status = 200, description = "Resource updated", body = Resource),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Resource not found", body = ErrorBody)
    )
)]
pub async fn update_resource(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(resource_id): Path<Uuid>,
    Json(input): Json<UpdateResource>,
) -> Result<Json<Resource>, AppError> {
    let input = UpdateResource {
        url: input.url.as_deref().map(validation::http_url).transpose()?,
        // An empty string is kept on purpose: it tells the database to clear the value
        title: input.title.map(|t| t.trim().to_string()),
        description: input.description.map(|d| d.trim().to_string()),
        visited: input.visited,
        shareable: input.shareable,
    };

    let resource = Resource::update(&state.db, user_id, resource_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(resource))
}

/// Delete a resource
#[utoipa::path(
    delete,
    path = "/resources/{resource_id}",
    tag = "Resources",
    security(("dev_user" = [])),
    params(
        ("resource_id" = Uuid, Path, description = "Resource to delete")
    ),
    responses(
        (status = 204, description = "Resource deleted"),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Resource not found", body = ErrorBody)
    )
)]
pub async fn delete_resource(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(resource_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if Resource::delete(&state.db, user_id, resource_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
