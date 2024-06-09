use crate::{database::Pool, services::user_service};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

pub fn get_user_routes() -> Router<Pool> {
    Router::new().route("/", post(create_user))
}

async fn create_user(
    State(pool): State<Pool>,
    Json(create_user): axum::extract::Json<CreateUserDTO>,
) -> impl IntoResponse {
    let user = user_service::create_user(create_user.name, create_user.username, &pool)
        .await
        .unwrap();

    let user_dto = UserDTO::from_user(user);
    return (StatusCode::CREATED, Json(user_dto));
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
