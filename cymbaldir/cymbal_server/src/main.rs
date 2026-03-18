use axum::{
    extract::{Path as AxumPath, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use sha2::{Digest, Sha384};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::fs;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Clone)]
struct Asset {
    data: Vec<u8>,
    hash: String,
    content_type: &'static str,
}

struct AppState {
    assets: HashMap<String, Asset>,
}

async fn load_asset(filename: &str, content_type: &'static str) -> Asset {
    let path = Path::new("src/assets").join(filename);
    let data = fs::read(&path)
        .await
        .expect(&format!("Failed to read asset {}", filename));

    let mut hasher = Sha384::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());

    info!("Pre-loaded {} (sha384: {})", filename, hash);

    Asset {
        data,
        hash,
        content_type,
    }
}

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

async fn get_asset_handler(
    State(state): State<Arc<AppState>>,
    AxumPath(asset_key): AxumPath<String>,
) -> impl IntoResponse {
    if let Some(asset) = state.assets.get(&asset_key) {
        (
            [
                (header::CONTENT_TYPE, asset.content_type),
                (header::HeaderName::from_static("x-sha384"), &asset.hash),
            ],
            asset.data.clone(),
        )
            .into_response()
    } else {
        warn!("Asset not found in cache: {}", asset_key);
        (
            StatusCode::NOT_FOUND,
            format!("Asset not found: {}", asset_key),
        )
            .into_response()
    }
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

    let mut assets = HashMap::new();

    assets.insert(
        "alice_blue".to_string(),
        load_asset("alice1a_blue.png", "image/png").await,
    );
    assets.insert(
        "alice_green".to_string(),
        load_asset("alice1a_green.png", "image/png").await,
    );
    assets.insert(
        "alice_red".to_string(),
        load_asset("alice1a_red.png", "image/png").await,
    );
    assets.insert(
        "note_c3".to_string(),
        load_asset(
            "chunk_003_c3_2f8a2198-4705-4b1f-b10d-00f024ab2d84.wav",
            "audio/wav",
        )
        .await,
    );
    assets.insert(
        "note_cs3".to_string(),
        load_asset(
            "chunk_004_c#3_9ab43356-8cdf-4863-b12e-496f34481017.wav",
            "audio/wav",
        )
        .await,
    );

    let state = Arc::new(AppState { assets });

    let app = Router::new()
        .route("/", get(hello_world))
        .route(
            "/alice/blue",
            get(|s| get_asset_handler(s, AxumPath("alice_blue".to_string()))),
        )
        .route(
            "/alice/green",
            get(|s| get_asset_handler(s, AxumPath("alice_green".to_string()))),
        )
        .route(
            "/alice/red",
            get(|s| get_asset_handler(s, AxumPath("alice_red".to_string()))),
        )
        .route(
            "/note/c3",
            get(|s| get_asset_handler(s, AxumPath("note_c3".to_string()))),
        )
        .route(
            "/note/cs3",
            get(|s| get_asset_handler(s, AxumPath("note_cs3".to_string()))),
        )
        .with_state(state);

    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
