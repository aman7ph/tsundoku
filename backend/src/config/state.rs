use std::{sync::Arc, time::Duration};

use sqlx::PgPool;

use super::AuthSettings;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth: Arc<AuthSettings>,
    /// Shared HTTP client, used to fetch Google's public signing keys
    pub http: reqwest::Client,
}

impl AppState {
    pub fn new(db: PgPool, auth: AuthSettings) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build the HTTP client");

        Self {
            db,
            auth: Arc::new(auth),
            http,
        }
    }
}