use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::services::errors::ServiceError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self.to_string()).into_response()
    }
}

impl From<ServiceError> for AppError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::NotFound => AppError::NotFound,
            ServiceError::InternalServerError(error) => AppError::InternalServerError(error),
        }
    }
}
