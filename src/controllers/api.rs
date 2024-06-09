use crate::controllers::{auth_routes, entity_routes, user_routes};
use crate::database::{self, Pool};

use axum::routing::get;
use axum::Router;

pub async fn get_v1_api() -> Router {
    let pool = database::get_database_pool().await.unwrap();

    add_test_data(&pool).await;

    Router::new()
        .route("/", get("NFC scanner api v1"))
        .nest("/entities", entity_routes::get_entity_routes())
        .nest("/users", user_routes::get_user_routes())
        .nest("/auth", auth_routes::get_auth_routes())
        .with_state(pool)
}

async fn add_test_data(pool: &Pool) {
    use crate::services::entity_service;

    let entity = entity_service::CreateEntity {
        tag_id: "049F3972FE4A80".to_string(),
        name: "Desert Eagle".to_string(),
        owner: "Mark".to_string(),
    };

    entity_service::create_entity(entity, pool).await.unwrap();
}
