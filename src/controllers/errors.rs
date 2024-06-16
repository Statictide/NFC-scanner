use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self.0);
        println!("{}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError(error)
    }
}
