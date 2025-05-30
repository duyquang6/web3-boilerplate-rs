use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! match_error_response {
    ($err:expr, $json:expr, $($ty:ty => $status:expr),* $(,)?) => {{
        $(
            if $err.is::<$ty>() {
                return ($status, axum::Json($json)).into_response();
            }
        )*
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::Json($json)).into_response()
    }};
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> response::Response {
        // check inner error type and return appropriate response, use match
        let json_response = json!({"error_msg": self.0.to_string()});

        match_error_response!(
            self.0,
            json_response,
            NotFoundError => StatusCode::NOT_FOUND,
            ValidateError => StatusCode::BAD_REQUEST,
        )
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
