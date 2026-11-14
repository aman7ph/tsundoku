use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

// One shared column list, joined into each query at compile time (SQLx needs literals)
macro_rules! columns {
    () => {
        "id, section_id, name, icon, position, created_at"
    };
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct Category {
    pub id: Uuid,
    pub section_id: Uuid,
    #[schema(example = "Rust")]
    pub name: String,
    #[schema(example = "emoji:🦀")]
    pub icon: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateCategory {
    #[schema(example = "Rust")]
    pub name: String,
    #[schema(example = "emoji:🦀")]
    pub icon: Option<String>,
}

/// Every field is optional: a missing field stays unchanged.
/// For `icon`, an empty string clears the value.
#[derive(Deserialize, ToSchema)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub position: Option<i32>,
}

impl Category {
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        section_id: Uuid,
    ) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(concat!(
            "SELECT ",
            columns!(),
            " FROM categories
             WHERE user_id = $1 AND section_id = $2
             ORDER BY position, created_at"
        ))
        .bind(user_id)
        .bind(section_id)
        .fetch_all(pool)
        .await
    }

    // Returns None when the section does not belong to this user
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        section_id: Uuid,
        name: &str,
        icon: Option<&str>,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(concat!(
            "INSERT INTO categories (user_id, section_id, name, icon)
             SELECT $1::uuid, s.id, $3::text, $4::text
             FROM sections s
             WHERE s.id = $2 AND s.user_id = $1
             RETURNING ",
            columns!()
        ))
        .bind(user_id)
        .bind(section_id)
        .bind(name)
        .bind(icon)
        .fetch_optional(pool)
        .await
    }

    // Returns None when the category does not exist or is not this user's
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
        input: &UpdateCategory,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(concat!(
            "UPDATE categories SET
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
        let result = sqlx::query("DELETE FROM categories WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
