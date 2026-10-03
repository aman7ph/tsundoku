mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod state;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let pool = db::create_pool(&config.database_url).await;
    let app = routes::create_router(AppState { db: pool });

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Tsundoku backend running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}