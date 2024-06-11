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

    let auth_response_dto = TokenResponse {
        token: session.token,
    };

    return (StatusCode::CREATED, Json(auth_response_dto)).into_response();
}

async fn _login_handler(// TypedHeader(Authorization(auth)): TypedHeader<Authorization<Basic>>,
) -> impl IntoResponse {
    // Extract the username and password
    let username = "auth.username()";
    let password = "auth.password()";

    // Validate credentials (this is just an example, replace with your own logic)
    if username == "user" && password == "password" {
        // Generate a token (for simplicity, we'll use the username)
        let token = username;
        let response = TokenResponse {
            token: token.to_string(),
        };

        (StatusCode::OK, Json(response)).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, Json("Invalid credentials")).into_response()
    }
}

#[derive(serde::Deserialize)]
struct AuthDTO {
    pub username: String,
}

#[derive(serde::Serialize)]
struct TokenResponse {
    token: String,
}
