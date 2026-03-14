use crate::{AppError, AppState, DURATION_HEADER};
use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::FromRow;
use std::sync::Arc;

pub const GET_ALL_USERS: &str = "/user";

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id_user: i64,
    pub email: String,
    pub ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id_user: i64,
    pub email: String,
    pub ctime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
pub struct UsersResponse {
    pub users: Vec<UserDto>,
}

pub async fn get_all_users(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("{GET_ALL_USERS}");

    let before = std::time::Instant::now();

    let rows: Vec<UserRow> = sqlx::query_as::<_, UserRow>(
        "SELECT id_user, email, ctime FROM public.nm_user ORDER BY id_user;",
    )
    .fetch_all(&state.db)
    .await?;

    let users = rows
        .into_iter()
        .map(|row| UserDto {
            id_user: row.id_user,
            email: row.email,
            ctime: row.ctime,
        })
        .collect::<Vec<_>>();

    let body = UsersResponse { users };

    Ok((
        StatusCode::OK,
        [(
            HeaderName::from_static(DURATION_HEADER),
            before.elapsed().as_millis().to_string(),
        )],
        Json(body),
    ))
}
