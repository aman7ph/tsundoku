use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

// One shared column list, joined into each query at compile time (SQLx needs literals)
macro_rules! columns {
    () => {
        "id, account_id, name, icon, position, created_at"
    };
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct SocialCategory {
    pub id: Uuid,
    pub account_id: Uuid,
    #[schema(example = "Music")]
    pub name: String,
    #[schema(example = "emoji:🎵")]
    pub icon: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSocialCategory {
    #[schema(example = "Music")]
    pub name: String,
    #[schema(example = "emoji:🎵")]
    pub icon: Option<String>,
}

/// Every field is optional: a missing field stays unchanged.
/// For `icon`, an empty string clears the value.
#[derive(Deserialize, ToSchema)]
pub struct UpdateSocialCategory {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub position: Option<i32>,
}

impl SocialCategory {
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        account_id: Uuid,
    ) -> Result<Vec<SocialCategory>, sqlx::Error> {
        sqlx::query_as::<_, SocialCategory>(concat!(
            "SELECT ",
            columns!(),
            " FROM social_categories
             WHERE user_id = $1 AND account_id = $2
             ORDER BY position, created_at"
        ))
        .bind(user_id)
        .bind(account_id)
        .fetch_all(pool)
        .await
    }

    // Returns None when the account does not belong to this user
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        account_id: Uuid,
        name: &str,
        icon: Option<&str>,
    ) -> Result<Option<SocialCategory>, sqlx::Error> {
        sqlx::query_as::<_, SocialCategory>(concat!(
            "INSERT INTO social_categories (user_id, account_id, name, icon)
             SELECT $1::uuid, a.id, $3::text, $4::text
             FROM accounts a
             WHERE a.id = $2 AND a.user_id = $1
             RETURNING ",
            columns!()
        ))
        .bind(user_id)
        .bind(account_id)
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
        input: &UpdateSocialCategory,
    ) -> Result<Option<SocialCategory>, sqlx::Error> {
        sqlx::query_as::<_, SocialCategory>(concat!(
            "UPDATE social_categories SET
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
        let result = sqlx::query("DELETE FROM social_categories WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
