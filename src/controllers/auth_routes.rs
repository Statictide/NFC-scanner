use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};

use crate::services::auth_service;

pub fn get_auth_routes() -> Router {
    Router::new().route("/", post(authenticate))
}

async fn authenticate(Json(auth_user): axum::extract::Json<AuthDTO>) -> Response {
    let session_result = auth_service::authenticate(auth_user.username).await;
    tracing::error!("Test");

    let Ok(session) = session_result else {
        tracing::error!("Failed to authenticate: {:?}", session_result);
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    let auth_response_dto: AuthResponseDTO = AuthResponseDTO {
        token: session.token,
    };

    return (StatusCode::CREATED, Json(auth_response_dto)).into_response();
}

#[derive(serde::Deserialize)]
struct AuthDTO {
    pub username: String,
}

#[derive(serde::Serialize)]
struct AuthResponseDTO {
    pub token: String,
}
