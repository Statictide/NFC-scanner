use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::services::auth_service;

use super::errors::AppResult;

pub fn get_auth_routes() -> Router {
    Router::new().route("/", post(authenticate))
}

async fn authenticate(
    Json(auth_user): axum::extract::Json<AuthDTO>,
) -> AppResult<impl IntoResponse> {
    let session_result = auth_service::authenticate(auth_user.username).await;

    let Ok(session) = session_result else {
        tracing::error!("Failed to authenticate: {:?}", session_result);
        return Ok((StatusCode::UNAUTHORIZED, "Unauthorized").into_response());
    };

    let auth_response_dto = TokenResponse {
        token: session.token,
    };

    return Ok((StatusCode::CREATED, Json(auth_response_dto)).into_response());
}

#[derive(serde::Deserialize)]
struct AuthDTO {
    pub username: String,
}

#[derive(serde::Serialize)]
struct TokenResponse {
    token: String,
}
