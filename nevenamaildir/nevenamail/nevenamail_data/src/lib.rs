use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::sync::Arc;

const DEFAULT_PORT: &str = "8000";
const DURATION_HEADER: &str = "x-took-ms";
const ERR_PORT: &str = "invalid port";
const GET_ALL_ROUTE: &str = "/version";
const PORT_CONST: &str = "PORT";
const DATABASE_URL_CONST: &str = "DATABASE_URL";
const STARTING_ON_PORT_MESSAGE: &str = "Starting on port";
const TCP_LISTENER_PREFIX: &str = "0.0.0.0";

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Payload {
    value: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct InsertBody {
    item: Payload,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, FromRow)]
struct VersionRow {
    id: i32,
    version: String,
    ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
struct VersionDto {
    id: i32,
    version: String,
    ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
struct VersionResponse {
    version: Vec<VersionDto>,
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
        .route(GET_ALL_ROUTE, get(get_all_versions))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{TCP_LISTENER_PREFIX}:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_all_versions(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("{GET_ALL_ROUTE}");

    let before = std::time::Instant::now();

    let rows: Vec<VersionRow> = sqlx::query_as::<_, VersionRow>(
        r#"
        SELECT id, version, ctime
        FROM pgversion
        ORDER BY id
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    let versions = rows
        .into_iter()
        .map(|row| VersionDto {
            id: row.id,
            version: row.version,
            ctime: row.ctime,
        })
        .collect::<Vec<_>>();

    let body = VersionResponse { version: versions };

    Ok((
        StatusCode::OK,
        [(
            HeaderName::from_static(DURATION_HEADER),
            before.elapsed().as_millis().to_string(),
        )],
        Json(body),
    ))
}
