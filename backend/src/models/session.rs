use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::config::TokenLifeSpan;

/// How long a refresh token stays valid
pub const REFRESH_TOKEN_DAYS: i32 = 30;

// SQL that hashes the token passed as parameter `$n`. Hashing in Postgres keeps the
// plain token out of our tables and needs no extra crate. A macro so that it can be
// joined into the query at compile time (SQLx needs literal queries).
macro_rules! hash_of {
    ($param:literal) => {
        concat!(
            "encode(sha256(convert_to(",
            $param,
            "::text, 'UTF8')), 'hex')"
        )
    };
}

#[derive(Deserialize, ToSchema)]
pub struct GoogleLogin {
    /// The ID token the app got from Google sign-in
    pub id_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenPair {
    /// Send as `Authorization: Bearer <access_token>`
    pub access_token: String,
    /// Exchange at POST /auth/refresh for a new pair. Single use.
    pub refresh_token: String,
    #[schema(example = "Bearer")]
    pub token_type: String,
    /// Seconds until the access token expires
    #[schema(example = 900)]
    pub expires_in: i64,
}

pub enum Rotation {
    Rotated { user_id: Uuid, refresh_token: String },
    Rejected,
}

/// A sign-in session: a chain of refresh tokens, each replaced by the next.
pub struct Session;

impl Session {
    /// Starts a new session and returns its first refresh token.
    pub async fn start(pool: &PgPool, user_id: Uuid) -> Result<String, sqlx::Error> {
        let mut conn = pool.acquire().await?;
        insert_token(&mut conn, user_id, Uuid::new_v4()).await
    }

    /// Exchanges a refresh token for a new one.
    ///
    /// A token can be used once. If an already-used token shows up again, someone may have
    /// copied it, so the whole session is revoked.
    pub async fn rotate(pool: &PgPool, presented: &str) -> Result<Rotation, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // One statement marks the token as used, so two requests cannot both succeed
        let taken = sqlx::query_as::<_, (Uuid, Uuid)>(concat!(
            "UPDATE refresh_tokens SET used_at = now()
             WHERE token_hash = ",
            hash_of!("$1"),
            " AND used_at IS NULL
               AND revoked_at IS NULL
               AND expires_at > now()
             RETURNING user_id, family_id"
        ))
        .bind(presented)
        .fetch_optional(&mut *tx)
        .await?;

        let Some((user_id, family_id)) = taken else {
            // Unknown, expired or revoked. If it was already used, end the whole session.
            sqlx::query(concat!(
                "UPDATE refresh_tokens SET revoked_at = now()
                 WHERE revoked_at IS NULL
                   AND family_id IN (
                       SELECT family_id FROM refresh_tokens
                       WHERE token_hash = ",
                hash_of!("$1"),
                " AND used_at IS NOT NULL)"
            ))
            .bind(presented)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            return Ok(Rotation::Rejected);
        };

        let refresh_token = insert_token(&mut tx, user_id, family_id).await?;
        tx.commit().await?;

        Ok(Rotation::Rotated {
            user_id,
            refresh_token,
        })
    }

    /// Ends the session the token belongs to. Unknown tokens are ignored.
    pub async fn end(pool: &PgPool, presented: &str) -> Result<(), sqlx::Error> {
        sqlx::query(concat!(
            "UPDATE refresh_tokens SET revoked_at = now()
             WHERE revoked_at IS NULL
               AND family_id IN (
                   SELECT family_id FROM refresh_tokens WHERE token_hash = ",
            hash_of!("$1"),
            ")"
        ))
        .bind(presented)
        .execute(pool)
        .await?;

        Ok(())
    }
}

// Creates a random token, stores its hash, and returns the plain token (the only time it exists)
async fn insert_token(
    conn: &mut PgConnection,
    user_id: Uuid,
    family_id: Uuid,
) -> Result<String, sqlx::Error> {
    // Two random UUIDs: 64 hex characters from the operating system's secure random source
    let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    
    let life = TokenLifeSpan::from_env();

    sqlx::query(concat!(
        "INSERT INTO refresh_tokens (user_id, family_id, token_hash, expires_at)
         VALUES ($1, $2, ",
        hash_of!("$3"),
        ", now() + make_interval(days => $4::int))"
    ))
    .bind(user_id)
    .bind(family_id)
    .bind(&token)
    .bind(life.refresh_token_life)
    .execute(conn)
    .await?;

    Ok(token)
}