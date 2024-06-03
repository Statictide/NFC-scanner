use crate::services::user_service;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::SqlitePool;

pub fn get_user_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

async fn create_user(
    State(pool): State<SqlitePool>,
    Json(create_user): axum::extract::Json<CreateUserDTO>,
) -> impl IntoResponse {
    let user = user_service::create_user(create_user.name, create_user.username, &pool)
        .await
        .unwrap();

    let user_dto = UserDTO::from_user(user);
    (StatusCode::CREATED, Json(user_dto))
}

async fn get_user(Path(id): Path<u32>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    let user = user_service::get_user(id, &pool).await.unwrap();
    let user_dto = UserDTO::from_user(user);
    (StatusCode::OK, Json(user_dto))
}

async fn update_user(
    Path(id): Path<u32>,
    State(pool): State<SqlitePool>,
    Json(update_user): Json<CreateUserDTO>,
) -> impl IntoResponse {
    let user = user_service::update_user(id, update_user.name, update_user.username, &pool)
        .await
        .unwrap();
    let user_dto = UserDTO::from_user(user);
    (StatusCode::OK, Json(user_dto))
}

async fn delete_user(Path(id): Path<u32>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    user_service::delete_user(id, &pool).await.unwrap();
    StatusCode::NO_CONTENT
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
