use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

// One shared column list, joined into each query at compile time (SQLx needs literals)
macro_rules! columns {
    () => {
        "id, category_id, url, title, description, shareable, created_at, updated_at"
    };
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct SocialLink {
    pub id: Uuid,
    pub category_id: Uuid,
    #[schema(example = "https://youtu.be/dQw4w9WgXcQ")]
    pub url: String,
    pub title: Option<String>,
    #[schema(example = "Great intro to the topic")]
    pub description: Option<String>,
    /// Worth sharing with other people
    pub shareable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSocialLink {
    #[schema(example = "https://youtu.be/dQw4w9WgXcQ")]
    pub url: String,
    pub title: Option<String>,
    #[schema(example = "Great intro to the topic")]
    pub description: Option<String>,
    #[serde(default)]
    pub shareable: bool,
}

/// Every field is optional: a missing field stays unchanged.
/// For `title` and `description`, an empty string clears the value.
#[derive(Deserialize, ToSchema)]
pub struct UpdateSocialLink {
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub shareable: Option<bool>,
}

impl SocialLink {
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        category_id: Uuid,
        shareable: Option<bool>,
    ) -> Result<Vec<SocialLink>, sqlx::Error> {
        sqlx::query_as::<_, SocialLink>(concat!(
            "SELECT ",
            columns!(),
            " FROM social_links
             WHERE user_id = $1
               AND category_id = $2
               AND ($3::boolean IS NULL OR shareable = $3)
             ORDER BY created_at DESC"
        ))
        .bind(user_id)
        .bind(category_id)
        .bind(shareable)
        .fetch_all(pool)
        .await
    }

    // Returns None when the category does not belong to this user
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        category_id: Uuid,
        input: &CreateSocialLink,
    ) -> Result<Option<SocialLink>, sqlx::Error> {
        sqlx::query_as::<_, SocialLink>(concat!(
            "INSERT INTO social_links (user_id, category_id, url, title, description, shareable)
             SELECT $1::uuid, c.id, $3::text, $4::text, $5::text, $6::boolean
             FROM social_categories c
             WHERE c.id = $2 AND c.user_id = $1
             RETURNING ",
            columns!()
        ))
        .bind(user_id)
        .bind(category_id)
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.description)
        .bind(input.shareable)
        .fetch_optional(pool)
        .await
    }

    // Returns None when the link does not exist or is not this user's
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        id: Uuid,
        input: &UpdateSocialLink,
    ) -> Result<Option<SocialLink>, sqlx::Error> {
        sqlx::query_as::<_, SocialLink>(concat!(
            "UPDATE social_links SET
                 url         = COALESCE($3::text, url),
                 title       = CASE WHEN $4::text IS NULL THEN title
                                    ELSE NULLIF($4::text, '') END,
                 description = CASE WHEN $5::text IS NULL THEN description
                                    ELSE NULLIF($5::text, '') END,
                 shareable   = COALESCE($6::boolean, shareable),
                 updated_at  = now()
             WHERE id = $1 AND user_id = $2
             RETURNING ",
            columns!()
        ))
        .bind(id)
        .bind(user_id)
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.description)
        .bind(input.shareable)
        .fetch_optional(pool)
        .await
    }

    // Returns true when a row was deleted
    pub async fn delete(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM social_links WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
