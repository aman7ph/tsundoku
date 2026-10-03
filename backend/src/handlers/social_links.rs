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
    models::social_link::{CreateSocialLink, SocialLink, UpdateSocialLink},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        validation,
    },
};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListSocialLinksParams {
    /// Only links with this shareable value
    pub shareable: Option<bool>,
}

/// List the links of a category, newest first
#[utoipa::path(
    get,
    path = "/social-categories/{category_id}/links",
    tag = "Social Links",
    security(("dev_user" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category to list links of"),
        ListSocialLinksParams
    ),
    responses(
        (status = 200, description = "Links in the category", body = Vec<SocialLink>),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn list_social_links(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
    Query(params): Query<ListSocialLinksParams>,
) -> Result<Json<Vec<SocialLink>>, AppError> {
    let links = SocialLink::list(&state.db, user_id, category_id, params.shareable).await?;
    Ok(Json(links))
}

/// Save a link into a category
#[utoipa::path(
    post,
    path = "/social-categories/{category_id}/links",
    tag = "Social Links",
    security(("dev_user" = [])),
    params(
        ("category_id" = Uuid, Path, description = "Category the link is saved in")
    ),
    request_body = CreateSocialLink,
    responses(
        (status = 201, description = "Link saved", body = SocialLink),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Category not found", body = ErrorBody)
    )
)]
pub async fn create_social_link(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(category_id): Path<Uuid>,
    Json(input): Json<CreateSocialLink>,
) -> Result<(StatusCode, Json<SocialLink>), AppError> {
    let input = CreateSocialLink {
        url: validation::http_url(&input.url)?,
        title: validation::clean_optional(input.title),
        description: validation::clean_optional(input.description),
        shareable: input.shareable,
    };

    let link = SocialLink::create(&state.db, user_id, category_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(link)))
}

/// Edit a link, or toggle shareable
#[utoipa::path(
    patch,
    path = "/social-links/{link_id}",
    tag = "Social Links",
    security(("dev_user" = [])),
    params(
        ("link_id" = Uuid, Path, description = "Link to update")
    ),
    request_body = UpdateSocialLink,
    responses(
        (status = 200, description = "Link updated", body = SocialLink),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Link not found", body = ErrorBody)
    )
)]
pub async fn update_social_link(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(link_id): Path<Uuid>,
    Json(input): Json<UpdateSocialLink>,
) -> Result<Json<SocialLink>, AppError> {
    let input = UpdateSocialLink {
        url: input.url.as_deref().map(validation::http_url).transpose()?,
        // An empty string is kept on purpose: it tells the database to clear the value
        title: input.title.map(|t| t.trim().to_string()),
        description: input.description.map(|d| d.trim().to_string()),
        shareable: input.shareable,
    };

    let link = SocialLink::update(&state.db, user_id, link_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(link))
}

/// Delete a link
#[utoipa::path(
    delete,
    path = "/social-links/{link_id}",
    tag = "Social Links",
    security(("dev_user" = [])),
    params(
        ("link_id" = Uuid, Path, description = "Link to delete")
    ),
    responses(
        (status = 204, description = "Link deleted"),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Link not found", body = ErrorBody)
    )
)]
pub async fn delete_social_link(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(link_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if SocialLink::delete(&state.db, user_id, link_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
