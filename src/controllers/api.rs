use crate::controllers::{entity_routes, user_routes};

use axum::routing::{get, post};
use axum::Router;

pub async fn get_v0_api() -> Router {
    Router::new()
        .route("/", get("NFC scanner api"))
        .nest("/entities", entity_routes::get_entity_routes())
        .nest("/users", user_routes::get_user_routes())
        .route("/check-for-update", post(update::check_for_update))
        .route("/app-update/check", get(update::check_for_update))
        .route("/app-update/download", get(update::download))
}

mod update {
    use axum::http::{header, StatusCode};
    use axum::response::IntoResponse;
    use axum::Json;
    use regex::Regex;

    use crate::controllers::errors::{AppError, AppResult};

    // Open the file at data/app-debug.apk, read it and return it as a response
    pub async fn download() -> AppResult<impl IntoResponse> {
        let headers = [
            (header::CONTENT_TYPE, "application/vnd.android.package-archive"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"app-debug.apk\""),
        ];
        
        let file = tokio::fs::read("/data/app-debug.apk").await.map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;
        // Set file name to app-debug.apk
        Ok((StatusCode::OK, (headers, file)))
    }

    pub async fn check_for_update(Json(body): axum::extract::Json<CheckForUpdateDTO>) -> AppResult<impl IntoResponse> {
        let app_version: Semver = Semver::from_str(&body.version).map_err(|e| AppError::BadRequest(e))?;
        let mandatory_version = Semver::new(0, 0, 1);
        let recommended_version = Semver::new(0, 0, 2);

        // mandatory < recommended < latest
        debug_assert!(mandatory_version < recommended_version);

        let update_not_recommended = recommended_version <= app_version;
        if update_not_recommended {
            return Ok((StatusCode::OK, Json(CheckForUpdateResponseDTO::empty())));
        }

        let update_recommended = mandatory_version <= app_version && app_version < recommended_version;
        if update_recommended {
            let response = CheckForUpdateResponseDTO {
                update_mandatory: false,
                update_recommended: true,
                title: Some("Update available".to_string()),
                message: Some("Assign To button has been fixed".to_string()),
                update_url: Some("https://nfc-scanner.fly.dev/api/v0/app-update/download".to_string()),
            };
            return Ok((StatusCode::OK, Json(response)));
        }

        let update_mandatory = app_version < mandatory_version;
        if update_mandatory {
            let response = CheckForUpdateResponseDTO {
                update_mandatory: true,
                update_recommended: true,
                title: Some("Update mandatory".to_string()),
                message: Some("Breaking change".to_string()),
                update_url: Some("https://nfc-scanner.fly.dev/api/v0/app-update/download".to_string()),
            };
            return Ok((StatusCode::OK, Json(response)));
        }

        Err(AppError::InternalServerError(anyhow::anyhow!("Unreachable code")))
    }

    struct Semver {
        major: u8,
        minor: u8,
        patch: u8,
    }

    const SEMVER_REGEX: &str = "^(?<major>[0-9]+)\\.(?<minor>[0-9]+)\\.(?<patch>[0-9]+)(?:-([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?(?:\\+[0-9A-Za-z-]+)?$";
    impl Semver {
        fn new(major: u8, minor: u8, patch: u8) -> Self {
            Self { major, minor, patch }
        }

        fn from_str(version: &str) -> Result<Self, String> {
            let regex = Regex::new(SEMVER_REGEX).expect("Bad regex");

            let cap = regex.captures(version).ok_or("Version is not a valid semver string")?;
            let major = cap
                .name("major")
                .ok_or("Version must be of semver format")?
                .as_str()
                .parse::<u8>()
                .map_err(|_| "Major is not a valid number")?;
            let minor = cap
                .name("minor")
                .ok_or("Version must be of semver format")?
                .as_str()
                .parse::<u8>()
                .map_err(|_| "Minor is not a valid number")?;
            let patch = cap
                .name("patch")
                .ok_or("Version must be of semver format")?
                .as_str()
                .parse::<u8>()
                .map_err(|_| "Patch is not a valid number")?;

            Ok(Self { major, minor, patch })
        }
    }

    impl PartialEq for Semver {
        fn eq(&self, other: &Self) -> bool {
            self.major == other.major && self.minor == other.minor && self.patch == other.patch
        }
    }

    impl PartialOrd for Semver {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            // https://doc.rust-lang.org/std/cmp/enum.Ordering.html
            if self.major > other.major {
                return Some(std::cmp::Ordering::Greater);
            } else if self.major < other.major {
                return Some(std::cmp::Ordering::Less);
            }

            if self.minor > other.minor {
                return Some(std::cmp::Ordering::Greater);
            } else if self.minor < other.minor {
                return Some(std::cmp::Ordering::Less);
            }

            if self.patch > other.patch {
                return Some(std::cmp::Ordering::Greater);
            } else if self.patch < other.patch {
                return Some(std::cmp::Ordering::Less);
            }

            return Some(std::cmp::Ordering::Equal);
        }
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
        pub update_mandatory: bool,
        pub update_recommended: bool,
        pub title: Option<String>,
        pub message: Option<String>,
        pub update_url: Option<String>,
    }

    impl CheckForUpdateResponseDTO {
        pub fn empty() -> Self {
            Self {
                update_mandatory: false,
                update_recommended: false,
                title: None,
                message: None,
                update_url: None,
            }
        }
    }
}
