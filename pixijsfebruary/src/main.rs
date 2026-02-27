use axum::{routing::get, Json, Router};
use rand::RngExt;
use serde::Serialize;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    random_number: u32,
}

async fn hello_world() -> Json<HelloResponse> {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random();

    Json(HelloResponse {
        message: "Hello, World!".to_string(),
        random_number,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api", get(hello_world))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
