mod config;
mod docs;
mod handlers;
mod models;
mod routes;
mod utils;

use config::{AppState, Config};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let db = config::create_pool(&config.database_url).await;

    if config.auth.dev_login_enabled {
        println!(
            "WARNING: ENABLE_DEV_LOGIN is on, so POST /auth/dev-login signs anyone in. \
             Never use this on a public server."
        );
    }

    let state = AppState::new(db, config.auth);
    let app = routes::create_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Tsundoku backend running on http://{addr}");
    println!("API docs at http://{addr}/docs");
    axum::serve(listener, app).await.unwrap();
}