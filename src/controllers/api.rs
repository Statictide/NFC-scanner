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
    use regex::Regex;
    use tokio::sync::OnceCell;

    use crate::controllers::errors::{AppError, AppResult};

    static SEMVER_REGEX: OnceCell<regex::Regex> = OnceCell::const_new();
    pub async fn check_for_update(Json(body): axum::extract::Json<CheckForUpdateDTO>) -> AppResult<impl IntoResponse> {
        let regex = SEMVER_REGEX
            .get_or_init(|| async { Regex::new("^(?<major>[0-9]+)\\.(?<minor>[0-9]+)\\.(?<patch>[0-9]+)$").unwrap() })
            .await;
        let cap = regex
            .captures(&body.version)
            .ok_or(AppError::BadRequest("Version is not a valid semver string".to_string()))?;
        let major = cap["major"].parse::<u8>().unwrap();
        let minor = cap["minor"].parse::<u8>().unwrap();
        let patch = cap["patch"].parse::<u8>().unwrap();
        // let alpha = cap["alpha"];

        // v0.0.0
        if major >= 0 && minor >= 0 && patch >= 0 {
            let response = CheckForUpdateResponseDTO {
                update_required: false,
                message: "All good".to_string(),
            };
            return Ok((StatusCode::OK, Json(response)));
        }

        let response = CheckForUpdateResponseDTO {
            update_required: true,
            message: "Please update".to_string(),
        };
        Ok((StatusCode::OK, Json(response)))
    }

    #[allow(dead_code)]
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
