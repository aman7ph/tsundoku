use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

// One shared column list, joined into each query at compile time (SQLx needs literals)
macro_rules! columns {
    () => {
        "id, key, name, icon, position, created_at"
    };
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct Platform {
    pub id: Uuid,
    /// Known platform identifier such as "youtube"; null for a custom platform
    #[schema(example = "youtube")]
    pub key: Option<String>,
    #[schema(example = "YouTube")]
    pub name: String,
    pub icon: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreatePlatform {
    /// Known platform identifier. Leave out for a custom platform. Cannot be changed later.
    #[schema(example = "youtube")]
    pub key: Option<String>,
    #[schema(example = "YouTube")]
    pub name: String,
    pub icon: Option<String>,
}

/// Every field is optional: a missing field stays unchanged.
/// For `icon`, an empty string clears the value.
#[derive(Deserialize, ToSchema)]
pub struct UpdatePlatform {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub position: Option<i32>,
}

impl Platform {
    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<Platform>, sqlx::Error> {
        sqlx::query_as::<_, Platform>(concat!(
            "SELECT ",
            columns!(),
            " FROM platforms
             WHERE user_id = $1
             ORDER BY position, created_at"
        ))
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        key: Option<&str>,
        name: &str,
        icon: Option<&str>,
    ) -> Result<Platform, sqlx::Error> {
        sqlx::query_as::<_, Platform>(concat!(
            "INSERT INTO platforms (user_id, key, name, icon)
             VALUES ($1, $2, $3, $4)
             RETURNING ",
            columns!()
        ))
        .bind(user_id)
        .bind(key)
        .bind(name)
        .bind(icon)
        .fetch_one(pool)
        .await
    }

    // Returns None when the platform does not exist or is not this user's
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
        input: &UpdatePlatform,
    ) -> Result<Option<Platform>, sqlx::Error> {
        sqlx::query_as::<_, Platform>(concat!(
            "UPDATE platforms SET
                 name     = COALESCE($3::text, name),
                 icon     = CASE WHEN $4::text IS NULL THEN icon
                                 ELSE NULLIF($4::text, '') END,
                 position = COALESCE($5::int, position)
             WHERE id = $1 AND user_id = $2
             RETURNING ",
            columns!()
        ))
        .bind(id)
        .bind(user_id)
        .bind(&input.name)
        .bind(&input.icon)
        .bind(input.position)
        .fetch_optional(pool)
        .await
    }

    // Returns true when a row was deleted
    pub async fn delete(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM platforms WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
