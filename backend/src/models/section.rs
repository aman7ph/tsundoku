use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, FromRow, ToSchema)]
pub struct Section {
    pub id: Uuid,
    #[schema(example = "Learning")]
    pub name: String,
    #[schema(example = "emoji:📚")]
    pub icon: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSection {
    #[schema(example = "Learning")]
    pub name: String,
    #[schema(example = "emoji:📚")]
    pub icon: Option<String>,
}

/// Every field is optional: a missing field stays unchanged.
/// For `icon`, an empty string clears the value.
#[derive(Deserialize, ToSchema)]
pub struct UpdateSection {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub position: Option<i32>,
}

impl Section {
    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<Section>, sqlx::Error> {
        sqlx::query_as::<_, Section>(
            "SELECT id, name, icon, position, created_at
             FROM sections
             WHERE user_id = $1
             ORDER BY position, created_at",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        name: &str,
        icon: Option<&str>,
    ) -> Result<Section, sqlx::Error> {
        sqlx::query_as::<_, Section>(
            "INSERT INTO sections (user_id, name, icon)
             VALUES ($1, $2, $3)
             RETURNING id, name, icon, position, created_at",
        )
        .bind(user_id)
        .bind(name)
        .bind(icon)
        .fetch_one(pool)
        .await
    }

    // Returns None when the section does not exist or is not this user's
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
        input: &UpdateSection,
    ) -> Result<Option<Section>, sqlx::Error> {
        sqlx::query_as::<_, Section>(
            "UPDATE sections SET
                 name     = COALESCE($3::text, name),
                 icon     = CASE WHEN $4::text IS NULL THEN icon
                                 ELSE NULLIF($4::text, '') END,
                 position = COALESCE($5::int, position)
             WHERE id = $1 AND user_id = $2
             RETURNING id, name, icon, position, created_at",
        )
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
        let result = sqlx::query("DELETE FROM sections WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
