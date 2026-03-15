use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub mod item;
pub mod user;
pub mod version;

const DEFAULT_PORT: &str = "8000";
pub const DURATION_HEADER: &str = "x-took-ms";
const ERR_PORT: &str = "invalid port";
const PORT_CONST: &str = "PORT";
const DATABASE_URL_CONST: &str = "DATABASE_URL";
const STARTING_ON_PORT_MESSAGE: &str = "Starting on port";
const TCP_LISTENER_PREFIX: &str = "0.0.0.0";

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Debug)]
pub enum AppError {
    Db(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Db(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Db(err) => {
                log::error!("database error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "internal server error"
                    })),
                )
                    .into_response()
            }
        }
    }
}

#[tokio::main]
pub async fn data_main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let port = std::env::var(PORT_CONST)
        .unwrap_or_else(|_| DEFAULT_PORT.into())
        .parse::<u16>()
        .expect(ERR_PORT);

    let database_url = std::env::var(DATABASE_URL_CONST).expect("DATABASE_URL must be set");

    let db = PgPool::connect(&database_url).await?;
    let state = Arc::new(AppState { db });

    log::info!("{STARTING_ON_PORT_MESSAGE} {port}");

    let app = Router::new()
        .route(version::GET_ALL_VERSIONS, get(version::get_all_versions))
        .route(user::GET_ALL_USERS, get(user::get_all_users))
        .route(item::GET_ALL_ITEMS, get(item::get_all_items))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{TCP_LISTENER_PREFIX}:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
