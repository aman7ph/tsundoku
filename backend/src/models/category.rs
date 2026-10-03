use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, FromRow, ToSchema)]
pub struct Category {
    pub id: Uuid,
    pub section_id: Uuid,
    /// Null for a top-level category
    pub parent_id: Option<Uuid>,
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
    /// Set this to create a subcategory; leave out for a top-level category
    pub parent_id: Option<Uuid>,
    #[schema(example = "emoji:🦀")]
    pub icon: Option<String>,
}

impl Category {
    // Children of `parent_id`, or the top level of the section when it is None
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        section_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "SELECT id, section_id, parent_id, name, icon, position, created_at
             FROM categories
             WHERE user_id = $1
               AND section_id = $2
               AND parent_id IS NOT DISTINCT FROM $3::uuid
             ORDER BY position, created_at",
        )
        .bind(user_id)
        .bind(section_id)
        .bind(parent_id)
        .fetch_all(pool)
        .await
    }

    // Returns None when the section (or the parent) does not belong to this user
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        section_id: Uuid,
        parent_id: Option<Uuid>,
        name: &str,
        icon: Option<&str>,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "INSERT INTO categories (user_id, section_id, parent_id, name, icon)
             SELECT $1::uuid, s.id, $3::uuid, $4::text, $5::text
             FROM sections s
             WHERE s.id = $2
               AND s.user_id = $1
               AND ($3::uuid IS NULL OR EXISTS (
                     SELECT 1 FROM categories p
                     WHERE p.id = $3::uuid
                       AND p.section_id = s.id
                       AND p.user_id = $1))
             RETURNING id, section_id, parent_id, name, icon, position, created_at",
        )
        .bind(user_id)
        .bind(section_id)
        .bind(parent_id)
        .bind(name)
        .bind(icon)
        .fetch_optional(pool)
        .await
    }
}
