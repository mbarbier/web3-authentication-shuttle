use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;


#[derive(Debug)]
pub enum AppError {
    Generic(String),
    //AuthenticationFail,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::Generic(m) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                m,
            ),
            //Self::AuthenticationFail => (StatusCode::UNAUTHORIZED, "Authentication failed".to_string()),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
