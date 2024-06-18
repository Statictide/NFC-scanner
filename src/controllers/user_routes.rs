use crate::services::user_service;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use super::errors::AppResult;

pub fn get_user_routes() -> Router {
    Router::new().route("/", post(create_user))
}

async fn create_user(Json(create_user): axum::extract::Json<CreateUserDTO>) -> AppResult<impl IntoResponse> {
    let user = user_service::create_user(create_user.name, create_user.username).await?;

    let user_dto = UserDTO::from_user(user);
    Ok((StatusCode::CREATED, Json(user_dto)))
}

#[derive(serde::Deserialize)]
struct CreateUserDTO {
    pub name: String,
    pub username: String,
}

#[derive(serde::Serialize)]
struct UserDTO {
    pub id: u32,
    pub name: String,
    pub username: String,
}

impl UserDTO {
    fn from_user(user: user_service::User) -> Self {
        UserDTO {
            id: user.id,
            name: user.name,
            username: user.username,
        }
    }
}
