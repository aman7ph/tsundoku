use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

// One shared column list. A macro (not a const) so `concat!` can join it into a
// compile-time string, which is what SQLx requires.
macro_rules! columns {
    () => {
        "id, section_id, parent_id, name, icon, position, created_at"
    };
}

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

/// Every field is optional: a missing field stays unchanged.
/// For `icon`, an empty string clears the value.
#[derive(Deserialize, ToSchema)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub position: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub struct MoveCategory {
    /// The new parent category. Null (or missing) moves it to the top level of its section.
    pub parent_id: Option<Uuid>,
}

impl Category {
    // Children of `parent_id`, or the top level of the section when it is None
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        section_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(concat!(
            "SELECT ",
            columns!(),
            " FROM categories
             WHERE user_id = $1
               AND section_id = $2
               AND parent_id IS NOT DISTINCT FROM $3::uuid
             ORDER BY position, created_at"
        ))
        .bind(user_id)
        .bind(section_id)
        .bind(parent_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(concat!(
            "SELECT ",
            columns!(),
            " FROM categories WHERE id = $1 AND user_id = $2"
        ))
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
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
        sqlx::query_as::<_, Category>(concat!(
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
             RETURNING ",
            columns!()
        ))
        .bind(user_id)
        .bind(section_id)
        .bind(parent_id)
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

    // Returns None when the move is not allowed: the new parent must be a category of
    // the same section, and must not be this category or anything inside it (a loop).
    pub async fn move_to(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(concat!(
            "UPDATE categories c SET parent_id = $3::uuid
             WHERE c.id = $1
               AND c.user_id = $2
               AND (
                 $3::uuid IS NULL
                 OR (
                   EXISTS (
                     SELECT 1 FROM categories p
                     WHERE p.id = $3::uuid
                       AND p.user_id = $2
                       AND p.section_id = c.section_id
                   )
                   AND $3::uuid NOT IN (
                     WITH RECURSIVE subtree AS (
                       SELECT id FROM categories WHERE id = $1
                       UNION ALL
                       SELECT ch.id FROM categories ch
                       JOIN subtree s ON ch.parent_id = s.id
                     )
                     SELECT id FROM subtree
                   )
                 )
               )
             RETURNING ",
            columns!()
        ))
        .bind(id)
        .bind(user_id)
        .bind(parent_id)
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
