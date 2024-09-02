use crate::controllers::{entity_routes, user_routes};

use axum::routing::{get, post};
use axum::Router;

pub async fn get_v0_api() -> Router {
    Router::new()
        .route("/", get("NFC scanner api"))
        .nest("/entities", entity_routes::get_entity_routes())
        .nest("/users", user_routes::get_user_routes())
        .route("/check-for-update", post(update::check_for_update))
}

mod update {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::Json;

    use crate::controllers::errors::AppResult;
    pub async fn check_for_update(Json(_): axum::extract::Json<CheckForUpdateDTO>) -> AppResult<impl IntoResponse> {
        let response = CheckForUpdateResponseDTO {
            update_required: false,
            message: "No update required".to_string(),
        };
        Ok((StatusCode::OK, Json(response)))
    }

    #[derive(serde::Deserialize)]
    pub struct CheckForUpdateDTO {
        pub version: String,
        pub platform: Platform,
    }
    
    #[derive(serde::Deserialize)]
    pub enum Platform {
        Android,
        Ios,
    }

    #[derive(serde::Serialize)]
    pub struct CheckForUpdateResponseDTO {
        pub update_required: bool,
        pub message: String,
    }
}
