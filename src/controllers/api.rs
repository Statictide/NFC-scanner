use super::{entity_routes, user_routes};
use crate::database;

use axum::routing::get;
use axum::Router;

pub async fn get_v1_api() -> Router {
    let pool = database::get_database_pool().await.unwrap();

    Router::new()
        .route("/", get("NFC scanner api v1"))
        .nest("/entities", entity_routes::get_entity_routes())
        .nest("/users", user_routes::get_user_routes())
        .with_state(pool)
}
