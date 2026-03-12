use std::{net::SocketAddr, sync::Arc, time::Instant};

use axum::{
    extract::State, http::{header::HeaderName, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use tower_http::services::ServeDir;
use turso::{Builder, Database};

const DEFAULT_PORT: u16 = 8000;
const PORT_ENV: &str = "PORT";

pub const DURATION_HEADER: &str = "x-took-ms";
pub const GET_ALL_VERSIONS: &str = "/version";
pub const API_VERSION: &str = "/api/version";

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "error": self.to_string()
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Serialize)]
struct VersionsResponse {
    versions: Vec<String>,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Database>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;

    conn.execute("CREATE TABLE IF NOT EXISTS version (version TEXT)", ())
        .await?;
    conn.execute("INSERT INTO version (version) VALUES ('0.1.0')", ())
        .await?;

    let state = AppState { db: Arc::new(db) };

    let port = std::env::var(PORT_ENV)
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    let app = Router::new()
        .route(GET_ALL_VERSIONS, get(get_all_versions))
        .route(API_VERSION, get(api_all_versions))
        .with_state(state)
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    log::info!("Starting server on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_all_versions() -> Result<impl IntoResponse, AppError> {
    let started_at = Instant::now();
    log::debug!("GET {GET_ALL_VERSIONS}");

    let index_html =
        std::fs::read_to_string("static/index.html").map_err(|_| AppError::Internal)?;

    let duration_ms = started_at.elapsed().as_millis().to_string();

    Ok((
        StatusCode::OK,
        [(HeaderName::from_static(DURATION_HEADER), duration_ms)],
        axum::response::Html(index_html),
    ))
}

async fn api_all_versions(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let started_at = Instant::now();
    log::debug!("GET {API_VERSION}");

    let conn = state.db.connect().map_err(|_| AppError::Internal)?;
    let mut rows = conn
        .query("SELECT version FROM version", ())
        .await
        .map_err(|_| AppError::Internal)?;

    let mut versions = Vec::new();
    while let Some(row) = rows.next().await.map_err(|_| AppError::Internal)? {
        let version: String = row.get(0).map_err(|_| AppError::Internal)?;
        versions.push(version);
    }

    let response = VersionsResponse { versions };

    let duration_ms = started_at.elapsed().as_millis().to_string();

    Ok((
        StatusCode::OK,
        [(HeaderName::from_static(DURATION_HEADER), duration_ms)],
        Json(response),
    ))
}
