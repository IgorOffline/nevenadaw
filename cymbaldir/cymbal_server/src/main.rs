use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use sha2::{Digest, Sha384};
use std::path::Path;
use tokio::fs;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

async fn get_alice_image(filename: &str) -> impl IntoResponse {
    let path = Path::new("src/assets").join(filename);
    match fs::read(&path).await {
        Ok(data) => {
            let mut hasher = Sha384::new();
            hasher.update(&data);
            let hash = format!("{:x}", hasher.finalize());

            info!("Serving {} (sha384: {})", filename, hash);

            (
                [
                    (header::CONTENT_TYPE, "image/png"),
                    (header::HeaderName::from_static("x-sha384"), &hash),
                ],
                data,
            )
                .into_response()
        }
        Err(e) => {
            warn!("Failed to read image {}: {}", filename, e);
            (StatusCode::NOT_FOUND, "Image not found").into_response()
        }
    }
}

async fn get_alice_blue() -> impl IntoResponse {
    get_alice_image("alice1a_blue.png").await
}

async fn get_alice_green() -> impl IntoResponse {
    get_alice_image("alice1a_green.png").await
}

async fn get_alice_red() -> impl IntoResponse {
    get_alice_image("alice1a_red.png").await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug,tower_http=debug,axum=debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/alice/blue", get(get_alice_blue))
        .route("/alice/green", get(get_alice_green))
        .route("/alice/red", get(get_alice_red));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
