use crate::controllers::{entity_routes, update, user_routes};

use axum::routing::{get, post};
use axum::Router;

use super::audit_log_routes;

pub async fn get_v0_api() -> Router {
    Router::new()
        .route("/", get("NFC scanner api"))
        .nest("/entities", entity_routes::get_entity_routes())
        .nest("/users", user_routes::get_user_routes())
        .nest("/history", audit_log_routes::get_history_routes())
        .route("/check-for-update", post(update::check_for_update)) // Delete me
        .route("/app-update/check", get(update::check_for_update))
        .route("/app-update/download", get(update::download))
}
