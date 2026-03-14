use crate::{AppError, AppState, DURATION_HEADER};
use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::FromRow;
use std::sync::Arc;

pub const GET_ALL_ITEMS: &str = "/item";

#[derive(Debug, FromRow)]
pub struct ItemRow {
    pub id_item: i64,
    pub value: i32,
    pub ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ItemDto {
    pub id_item: i64,
    pub value: i32,
    pub ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
pub struct ItemsResponse {
    pub items: Vec<ItemDto>,
}

pub async fn get_all_items(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("{GET_ALL_ITEMS}");

    let before = std::time::Instant::now();

    let rows: Vec<ItemRow> = sqlx::query_as::<_, ItemRow>(
        "SELECT id_item, value, ctime FROM public.nm_item ORDER BY id_item;",
    )
    .fetch_all(&state.db)
    .await?;

    let items = rows
        .into_iter()
        .map(|row| ItemDto {
            id_item: row.id_item,
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
