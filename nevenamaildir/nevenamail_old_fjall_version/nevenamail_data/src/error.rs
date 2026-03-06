use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Axum(#[from] axum::Error),

    #[error(transparent)]
    Fjall(#[from] fjall::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}
