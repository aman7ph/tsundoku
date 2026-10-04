use std::env;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub auth: AuthSettings,
}

pub struct AuthSettings {
    /// Secret used to sign access tokens
    pub jwt_secret: String,
    /// The Google *Web* client ID that ID tokens must have been issued for
    pub google_client_id: String,
    /// Enables POST /auth/dev-login. Development only.
    pub dev_login_enabled: bool,
}

impl AuthSettings {
    fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
        assert!(
            jwt_secret.len() >= 32,
            "JWT_SECRET must be at least 32 characters long"
        );

        Self {
            jwt_secret,
            google_client_id: env::var("GOOGLE_CLIENT_ID")
                .expect("GOOGLE_CLIENT_ID must be set in .env"),
            dev_login_enabled: env::var("ENABLE_DEV_LOGIN").is_ok_and(|v| v == "true"),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env"),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            auth: AuthSettings::from_env(),
        }
    }
}