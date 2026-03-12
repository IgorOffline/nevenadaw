use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State, http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json,
    Router,
};
use tower_http::services::ServeDir;

const PORT_ENV: &str = "PORT";
const DEFAULT_PORT: u16 = 8000;

#[derive(Clone)]
struct AppState {
    index_html: Arc<str>,
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
enum AppError {
    #[error("internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "error": "internal server error"
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let port = std::env::var(PORT_ENV)
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    let state = AppState {
        index_html: Arc::from(std::fs::read_to_string("static/index.html")?),
    };

    let app = Router::new()
        .route("/", get(get_page_index))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    log::info!("Starting server on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_page_index(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    log::debug!("get_page_index");
    Ok(Html(state.index_html.to_string()))
}
