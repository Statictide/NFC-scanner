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
        let (major, minor, patch) = parse_semver(body).await?;

        // v0.0.0
        #[allow(unused_comparisons)]
        if major >= 0 && minor >= 0 && patch >= 0 {
            let response = CheckForUpdateResponseDTO {
                update_required: false,
                update_recommended: false,
                message: "All good".to_string(),
            };
            return Ok((StatusCode::OK, Json(response)));
        }

        let response = CheckForUpdateResponseDTO {
            update_required: false,
            update_recommended: true,
            message: "Please update".to_string(),
        };
        Ok((StatusCode::OK, Json(response)))
    }

    async fn parse_semver(body: CheckForUpdateDTO) -> AppResult<(u8, u8, u8)> {
        let regex = SEMVER_REGEX
            .get_or_init(|| async { Regex::new("^(?<major>[0-9]+)\\.(?<minor>[0-9]+)\\.(?<patch>[0-9]+)(?:-([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?(?:\\+[0-9A-Za-z-]+)?$").unwrap() })
            .await;
        let cap = regex
            .captures(&body.version)
            .ok_or(AppError::BadRequest("Version is not a valid semver string".to_string()))?;
        let major = cap.name("major")
            .ok_or(AppError::BadRequest("".to_string()))?
            .as_str()
            .parse::<u8>()
            .map_err(|_| AppError::BadRequest("Major is not a valid number".to_string()))?;
        let minor = cap.name("minor")
            .ok_or(AppError::BadRequest("".to_string()))?
            .as_str()
            .parse::<u8>()
            .map_err(|_| AppError::BadRequest("Minor is not a valid number".to_string()))?;
        let patch = cap.name("patch")
            .ok_or(AppError::BadRequest("".to_string()))?
            .as_str()
            .parse::<u8>()
            .map_err(|_| AppError::BadRequest("Patch is not a valid number".to_string()))?;

        Ok((major, minor, patch))
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
        pub update_recommended: bool,
        pub message: String,
    }
}
