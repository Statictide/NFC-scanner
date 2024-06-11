use crate::controllers::{auth_routes, entity_routes, user_routes};
use crate::database::db;
use crate::services::user_service;

use axum::routing::get;
use axum::Router;

pub async fn get_v1_api() -> Router {
    // Initialize database pool upfront
    db::init_database_pool(db::DatabaseType::InMemory).await.expect("Failed to initialize database connection");
    add_test_data().await;

    Router::new()
        .route("/", get("NFC scanner api v1"))
        .nest("/entities", entity_routes::get_entity_routes())
        .nest("/users", user_routes::get_user_routes())
        .nest("/auth", auth_routes::get_auth_routes())
}

async fn add_test_data() {
    use crate::services::entity_service;

    let user = user_service::create_user("Mark".to_string(), "Static".to_string())
        .await
        .unwrap();

    let entity = entity_service::CreateEntity {
        tag_id: "049F3972FE4A80".to_string(),
        name: "Desert Eagle".to_string(),
        user_id: user.id,
    };

    entity_service::create_entity(entity).await.unwrap();
}

