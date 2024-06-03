use crate::services::entity_service::{self, CreateEntity, Entity};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::SqlitePool;

pub fn get_entity_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", post(create_entity).get(get_entities))
        .route(
            "/:id",
            get(get_entity).put(update_entity).delete(delete_entity),
        )
}

async fn create_entity(
    State(pool): State<SqlitePool>,
    Json(create_entity): Json<CreateEntityDTO>,
) -> impl IntoResponse {
    let entity = entity_service::create_entity(create_entity.into_create_entity(), &pool)
        .await
        .unwrap();

    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::CREATED, Json(entity_dto))
}

async fn get_entity(Path(id): Path<u32>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    let entity = entity_service::get_entity(id, &pool).await.unwrap();
    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::OK, Json(entity_dto))
}

async fn get_entities(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let entities = entity_service::get_entities(&pool).await.unwrap();
    let entities_dto = entities
        .into_iter()
        .map(EntityDTO::from_entity)
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(entities_dto))
}

async fn update_entity(
    Path(id): Path<u32>,
    State(pool): State<SqlitePool>,
    Json(update_entity): Json<CreateEntityDTO>,
) -> impl IntoResponse {
    let entity = entity_service::update_entity(id, update_entity.into_create_entity(), &pool)
        .await
        .unwrap();
    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::OK, Json(entity_dto))
}

async fn delete_entity(Path(id): Path<u32>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    entity_service::delete_entity(id, &pool).await.unwrap();
    StatusCode::NO_CONTENT
}

#[derive(serde::Deserialize)]
pub struct CreateEntityDTO {
    tag_id: String,
    name: String,
    owner: String,
}

impl CreateEntityDTO {
    pub fn into_create_entity(self) -> CreateEntity {
        CreateEntity {
            tag_id: self.tag_id,
            name: self.name,
            owner: self.owner,
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityDTO {
    id: u32,
    name: String,
    owner: String,
}

impl EntityDTO {
    fn from_entity(entity: Entity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            owner: entity.owner,
        }
    }
}
