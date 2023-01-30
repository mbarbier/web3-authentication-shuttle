use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::{event, Level};

#[derive(Debug)]
pub enum AppError {
    Generic(String),
    DbError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::Generic(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
            Self::DbError => (StatusCode::INTERNAL_SERVER_ERROR, "db error".to_string()),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        event!(Level::ERROR, "DB error {}", value);
        AppError::DbError
    }
}
