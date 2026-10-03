use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, FromRow, ToSchema)]
pub struct Resource {
    pub id: Uuid,
    pub category_id: Uuid,
    #[schema(example = "https://docs.rs/axum")]
    pub url: String,
    #[schema(example = "axum documentation")]
    pub title: Option<String>,
    #[schema(example = "Good reference for extractors and routing")]
    pub description: Option<String>,
    /// You have actually opened and read it
    pub visited: bool,
    /// Worth sharing with other people
    pub shareable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateResource {
    #[schema(example = "https://docs.rs/axum")]
    pub url: String,
    #[schema(example = "axum documentation")]
    pub title: Option<String>,
    #[schema(example = "Good reference for extractors and routing")]
    pub description: Option<String>,
    #[serde(default)]
    pub visited: bool,
    #[serde(default)]
    pub shareable: bool,
}

/// Every field is optional: a missing field stays unchanged.
/// For `title` and `description`, an empty string clears the value.
#[derive(Deserialize, ToSchema)]
pub struct UpdateResource {
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub visited: Option<bool>,
    pub shareable: Option<bool>,
}

impl Resource {
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        category_id: Uuid,
        visited: Option<bool>,
        shareable: Option<bool>,
    ) -> Result<Vec<Resource>, sqlx::Error> {
        sqlx::query_as::<_, Resource>(
            "SELECT id, category_id, url, title, description, visited, shareable,
                    created_at, updated_at
             FROM resources
             WHERE user_id = $1
               AND category_id = $2
               AND ($3::boolean IS NULL OR visited = $3)
               AND ($4::boolean IS NULL OR shareable = $4)
             ORDER BY created_at DESC",
        )
        .bind(user_id)
        .bind(category_id)
        .bind(visited)
        .bind(shareable)
        .fetch_all(pool)
        .await
    }

    // Returns None when the category does not belong to this user
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        category_id: Uuid,
        input: &CreateResource,
    ) -> Result<Option<Resource>, sqlx::Error> {
        sqlx::query_as::<_, Resource>(
            "INSERT INTO resources
                 (user_id, category_id, url, title, description, visited, shareable)
             SELECT $1::uuid, c.id, $3::text, $4::text, $5::text, $6::boolean, $7::boolean
             FROM categories c
             WHERE c.id = $2 AND c.user_id = $1
             RETURNING id, category_id, url, title, description, visited, shareable,
                       created_at, updated_at",
        )
        .bind(user_id)
        .bind(category_id)
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.description)
        .bind(input.visited)
        .bind(input.shareable)
        .fetch_optional(pool)
        .await
    }

    // Returns None when the resource does not exist or is not this user's
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
        input: &UpdateResource,
    ) -> Result<Option<Resource>, sqlx::Error> {
        sqlx::query_as::<_, Resource>(
            "UPDATE resources SET
                 url         = COALESCE($3::text, url),
                 title       = CASE WHEN $4::text IS NULL THEN title
                                    ELSE NULLIF($4::text, '') END,
                 description = CASE WHEN $5::text IS NULL THEN description
                                    ELSE NULLIF($5::text, '') END,
                 visited     = COALESCE($6::boolean, visited),
                 shareable   = COALESCE($7::boolean, shareable),
                 updated_at  = now()
             WHERE id = $1 AND user_id = $2
             RETURNING id, category_id, url, title, description, visited, shareable,
                       created_at, updated_at",
        )
        .bind(id)
        .bind(user_id)
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.description)
        .bind(input.visited)
        .bind(input.shareable)
        .fetch_optional(pool)
        .await
    }

    // Returns true when a row was deleted
    pub async fn delete(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM resources WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
