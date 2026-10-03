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
    let app = routes::create_router(AppState { db });

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Tsundoku backend running on http://{addr}");
    println!("API docs at http://{addr}/docs");
    axum::serve(listener, app).await.unwrap();
}
