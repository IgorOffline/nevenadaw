use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;

const DEFAULT_PORT: &str = "8000";
const DURATION_HEADER: &str = "x-took-ms";
const ERR_PORT: &str = "invalid port";
const GET_ALL_VERSIONS: &str = "/version";
const GET_ALL_ITEMS: &str = "/item";
const PORT_CONST: &str = "PORT";
const DATABASE_URL_CONST: &str = "DATABASE_URL";
const STARTING_ON_PORT_MESSAGE: &str = "Starting on port";
const TCP_LISTENER_PREFIX: &str = "0.0.0.0";

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, FromRow)]
struct VersionRow {
    version_id: i32,
    version: String,
    ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
struct VersionDto {
    version_id: i32,
    version: String,
    ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
struct VersionsResponse {
    versions: Vec<VersionDto>,
}

#[derive(Debug, FromRow)]
struct ItemRow {
    item_id: i64,
    value: i32,
    ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
struct ItemDto {
    item_id: i64,
    value: i32,
    ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
struct ItemsResponse {
    items: Vec<ItemDto>,
}

#[derive(Debug)]
enum AppError {
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
        .route(GET_ALL_VERSIONS, get(get_all_versions))
        .route(GET_ALL_ITEMS, get(get_all_items))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{TCP_LISTENER_PREFIX}:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_all_versions(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("{GET_ALL_VERSIONS}");

    let before = std::time::Instant::now();

    let rows: Vec<VersionRow> = sqlx::query_as::<_, VersionRow>(
        "SELECT version_id, version, ctime FROM public.version ORDER BY version_id",
    )
    .fetch_all(&state.db)
    .await?;

    let versions = rows
        .into_iter()
        .map(|row| VersionDto {
            version_id: row.version_id,
            version: row.version,
            ctime: row.ctime,
        })
        .collect::<Vec<_>>();

    let body = VersionsResponse { versions };

    Ok((
        StatusCode::OK,
        [(
            HeaderName::from_static(DURATION_HEADER),
            before.elapsed().as_millis().to_string(),
        )],
        Json(body),
    ))
}

async fn get_all_items(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    log::debug!("{GET_ALL_ITEMS}");

    let before = std::time::Instant::now();

    let rows: Vec<ItemRow> = sqlx::query_as::<_, ItemRow>(
        "SELECT item_id, value, ctime FROM public.item ORDER BY item_id",
    )
    .fetch_all(&state.db)
    .await?;

    let items = rows
        .into_iter()
        .map(|row| ItemDto {
            item_id: row.item_id,
            value: row.value,
            ctime: row.ctime,
        })
        .collect::<Vec<_>>();

    let body = ItemsResponse { items };

    Ok((
        StatusCode::OK,
        [(
            HeaderName::from_static(DURATION_HEADER),
            before.elapsed().as_millis().to_string(),
        )],
        Json(body),
    ))
}
