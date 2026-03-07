use crate::{AppError, AppState, DURATION_HEADER};
use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::FromRow;
use std::sync::Arc;

pub const GET_ALL_VERSIONS: &str = "/version";

#[derive(Debug, FromRow)]
pub struct VersionRow {
    pub id_version: i32,
    pub version: String,
    pub ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct VersionDto {
    pub id_version: i32,
    pub version: String,
    pub ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
pub struct VersionsResponse {
    pub versions: Vec<VersionDto>,
}

pub async fn get_all_versions(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("{GET_ALL_VERSIONS}");

    let before = std::time::Instant::now();

    let rows: Vec<VersionRow> = sqlx::query_as::<_, VersionRow>(
        "SELECT id_version, version, ctime FROM public.nm_version ORDER BY id_version;",
    )
    .fetch_all(&state.db)
    .await?;

    let versions = rows
        .into_iter()
        .map(|row| VersionDto {
            id_version: row.id_version,
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
