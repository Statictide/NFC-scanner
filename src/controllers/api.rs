use crate::controllers::{auth_routes, entity_routes, user_routes};
use crate::database::db;
use crate::services::user_service;

use axum::routing::get;
use axum::Router;

pub async fn get_v1_api() -> Router {
    // Initialize database pool upfront
    db::init_database_pool(db::DatabaseType::InMemory)
        .await
        .expect("Failed to initialize database connection");

    add_test_data().await;

    Router::new()
        .route("/", get("NFC scanner api v1"))
        .nest("/entities", entity_routes::get_entity_routes())
        .nest("/users", user_routes::get_user_routes())
        .nest("/sessions", auth_routes::get_auth_routes())
}

async fn add_test_data() {
    use crate::services::entity_service::create_entity;
    use crate::services::entity_service::CreateEntity;

    let user = user_service::create_user("Mark".to_string(), "Static".to_string())
        .await
        .unwrap();

    let parent = create_entity(CreateEntity {
        tag_uid: "0".to_string(),
        name: "Original Grandparent".to_string(),
        user_id: user.id,
        parent_id: None,
    })
    .await
    .unwrap();

    let entity = create_entity(CreateEntity {
        tag_uid: "049F3972FE4A80".to_string(),
        name: "Main entity 1".to_string(),
        user_id: user.id,
        parent_id: Some(parent.id),
    })
    .await
    .unwrap();

    let _child1 = create_entity(CreateEntity {
        tag_uid: "1".to_string(),
        name: "Child 1".to_string(),
        user_id: user.id,
        parent_id: Some(entity.id),
    })
    .await
    .unwrap();

    let _child2 = create_entity(CreateEntity {
        tag_uid: "2".to_string(),
        name: "Child 2".to_string(),
        user_id: user.id,
        parent_id: Some(entity.id),
    })
    .await
    .unwrap();

    let _ = create_entity(CreateEntity {
        tag_uid: "043A9F52A84A81".to_string(),
        name: "Main entity 2".to_string(),
        user_id: user.id,
        parent_id: None,
    })
    .await
    .unwrap();
}
