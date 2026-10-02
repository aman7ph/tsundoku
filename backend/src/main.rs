use axum::{routing::get, Router};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Tsundoku backend running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap()

}

// A handler: runs when someone visits "/"
async fn hello() -> &'static str {
    "Hello from Tsundoku! 積ん読"
}