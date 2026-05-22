use axum::{response::IntoResponse, Json, http::StatusCode};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => {
                tracing::error!(error = %msg, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string())
            }
        };

        Json(json!({
            "error": message,
            "status": status.as_u16()
        })).into_response()
    }
}

impl From<{{workspace_name}}_core::Error> for AppError {
    fn from(e: {{workspace_name}}_core::Error) -> Self {
        match e {
            CoreError::NotFound(_) => AppError::NotFound,
            CoreError::Validation(v) => AppError::ValidationError(v.to_string()),
            CoreError::Internal(msg) => AppError::Internal(msg),
        }
    }
}