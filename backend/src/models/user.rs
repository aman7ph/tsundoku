use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    #[schema(example = "aman@example.com")]
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn find(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, name, avatar_url, created_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// Finds the user by Google's id or creates them, refreshing their profile. Returns the id.
    pub async fn upsert_google(
        pool: &PgPool,
        google_sub: &str,
        email: &str,
        name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<Uuid, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO users (google_sub, email, name, avatar_url)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (google_sub) DO UPDATE
                 SET email = EXCLUDED.email,
                     name = EXCLUDED.name,
                     avatar_url = EXCLUDED.avatar_url
             RETURNING id",
        )
        .bind(google_sub)
        .bind(email)
        .bind(name)
        .bind(avatar_url)
        .fetch_one(pool)
        .await
    }
}
