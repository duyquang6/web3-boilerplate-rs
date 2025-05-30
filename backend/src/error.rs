use axum::{
    Json,
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug)]
pub struct AppError(anyhow::Error);
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> response::Response {
        let json_response = json!({"error_msg": self.0.to_string()});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Debug, Error)]
#[error("Not found, details: {0}")]
pub struct NotFoundError(pub String);

#[derive(Debug, Error)]
#[error("Validate error: {0}")]
pub struct ValidateError(pub String);

pub type Result<T> = std::result::Result<T, AppError>;
