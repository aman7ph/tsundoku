use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    config::AppState,
    models::account::{Account, CreateAccount, UpdateAccount},
    utils::{
        auth::CurrentUser,
        error::{AppError, ErrorBody},
        validation,
    },
};

/// List the accounts of a platform
#[utoipa::path(
    get,
    path = "/platforms/{platform_id}/accounts",
    tag = "Accounts",
    security(("bearer" = [])),
    params(
        ("platform_id" = Uuid, Path, description = "Platform to list accounts of")
    ),
    responses(
        (status = 200, description = "Accounts of the platform", body = Vec<Account>),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody)
    )
)]
pub async fn list_accounts(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(platform_id): Path<Uuid>,
) -> Result<Json<Vec<Account>>, AppError> {
    let accounts = Account::list(&state.db, user_id, platform_id).await?;
    Ok(Json(accounts))
}

/// Add an account to a platform
#[utoipa::path(
    post,
    path = "/platforms/{platform_id}/accounts",
    tag = "Accounts",
    security(("bearer" = [])),
    params(
        ("platform_id" = Uuid, Path, description = "Platform the account belongs to")
    ),
    request_body = CreateAccount,
    responses(
        (status = 201, description = "Account created", body = Account),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Platform not found", body = ErrorBody),
        (status = 409, description = "An account with this name already exists on the platform", body = ErrorBody)
    )
)]
pub async fn create_account(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(platform_id): Path<Uuid>,
    Json(input): Json<CreateAccount>,
) -> Result<(StatusCode, Json<Account>), AppError> {
    let name = validation::required_name(&input.name)?;

    let account = Account::create(&state.db, user_id, platform_id, &name)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(account)))
}

/// Rename an account or reorder it
#[utoipa::path(
    patch,
    path = "/accounts/{account_id}",
    tag = "Accounts",
    security(("bearer" = [])),
    params(
        ("account_id" = Uuid, Path, description = "Account to update")
    ),
    request_body = UpdateAccount,
    responses(
        (status = 200, description = "Account updated", body = Account),
        (status = 400, description = "Invalid input", body = ErrorBody),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Account not found", body = ErrorBody),
        (status = 409, description = "An account with this name already exists on the platform", body = ErrorBody)
    )
)]
pub async fn update_account(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(account_id): Path<Uuid>,
    Json(input): Json<UpdateAccount>,
) -> Result<Json<Account>, AppError> {
    let input = UpdateAccount {
        name: input
            .name
            .as_deref()
            .map(validation::required_name)
            .transpose()?,
        position: input.position,
    };

    let account = Account::update(&state.db, user_id, account_id, &input)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(account))
}

/// Delete an account
#[utoipa::path(
    delete,
    path = "/accounts/{account_id}",
    tag = "Accounts",
    security(("bearer" = [])),
    params(
        ("account_id" = Uuid, Path, description = "Account to delete")
    ),
    responses(
        (status = 204, description = "Account deleted"),
        (status = 401, description = "Missing or invalid credentials", body = ErrorBody),
        (status = 404, description = "Account not found", body = ErrorBody)
    )
)]
pub async fn delete_account(
    State(state): State<AppState>,
    CurrentUser(user_id): CurrentUser,
    Path(account_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if Account::delete(&state.db, user_id, account_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
