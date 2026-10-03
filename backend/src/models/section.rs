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
}
