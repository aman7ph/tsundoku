use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

// One shared column list, joined into each query at compile time (SQLx needs literals)
macro_rules! columns {
    () => {
        "id, platform_id, name, position, created_at"
    };
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct Account {
    pub id: Uuid,
    pub platform_id: Uuid,
    /// Handle or label of the account
    #[schema(example = "@aman_main")]
    pub name: String,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAccount {
    #[schema(example = "@aman_main")]
    pub name: String,
}

/// Every field is optional: a missing field stays unchanged.
#[derive(Deserialize, ToSchema)]
pub struct UpdateAccount {
    pub name: Option<String>,
    pub position: Option<i32>,
}

impl Account {
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        platform_id: Uuid,
    ) -> Result<Vec<Account>, sqlx::Error> {
        sqlx::query_as::<_, Account>(concat!(
            "SELECT ",
            columns!(),
            " FROM accounts
             WHERE user_id = $1 AND platform_id = $2
             ORDER BY position, created_at"
        ))
        .bind(user_id)
        .bind(platform_id)
        .fetch_all(pool)
        .await
    }

    // Returns None when the platform does not belong to this user
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        platform_id: Uuid,
        name: &str,
    ) -> Result<Option<Account>, sqlx::Error> {
        sqlx::query_as::<_, Account>(concat!(
            "INSERT INTO accounts (user_id, platform_id, name)
             SELECT $1::uuid, p.id, $3::text
             FROM platforms p
             WHERE p.id = $2 AND p.user_id = $1
             RETURNING ",
            columns!()
        ))
        .bind(user_id)
        .bind(platform_id)
        .bind(name)
        .fetch_optional(pool)
        .await
    }

    // Returns None when the account does not exist or is not this user's
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
        input: &UpdateAccount,
    ) -> Result<Option<Account>, sqlx::Error> {
        sqlx::query_as::<_, Account>(concat!(
            "UPDATE accounts SET
                 name     = COALESCE($3::text, name),
                 position = COALESCE($4::int, position)
             WHERE id = $1 AND user_id = $2
             RETURNING ",
            columns!()
        ))
        .bind(id)
        .bind(user_id)
        .bind(&input.name)
        .bind(input.position)
        .fetch_optional(pool)
        .await
    }

    // Returns true when a row was deleted
    pub async fn delete(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM accounts WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
