use crate::{
    database::Pool,
    services::entity_service::{self, CreateEntity, Entity},
};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

pub fn get_entity_routes() -> Router<Pool> {
    Router::new()
        .route("/", post(create_entity).get(get_entities))
        .route(
            "/:id",
            get(get_entity).put(update_entity).delete(delete_entity),
        )
}

async fn create_entity(
    State(pool): State<Pool>,
    Json(create_entity): Json<CreateEntityDTO>,
) -> impl IntoResponse {
    let entity = entity_service::create_entity(create_entity.into_create_entity(), &pool)
        .await
        .unwrap();

    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::CREATED, Json(entity_dto))
}

async fn get_entity(Path(id): Path<u32>, State(pool): State<Pool>) -> impl IntoResponse {
    let entity = entity_service::get_entity(id, &pool).await.unwrap();
    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::OK, Json(entity_dto))
}

async fn get_entities(
    Query(query): Query<TagIdQuery>,
    State(pool): State<Pool>,
) -> impl IntoResponse {
    if let Some(tag_id) = query.tag_id {
        let entity_option = entity_service::get_entity_by_tag_id(tag_id, &pool)
            .await
            .unwrap();

        let Some(entity) = entity_option else {
            return (StatusCode::OK, Json(vec![]));
        };

        let entity_dto = EntityDTO::from_entity(entity);
        return (StatusCode::OK, Json(vec![entity_dto]));
    }

    let entities = entity_service::get_entities(&pool).await.unwrap();
    let entities_dto = entities
        .into_iter()
        .map(EntityDTO::from_entity)
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(entities_dto))
}

#[derive(Deserialize)]
struct TagIdQuery {
    pub tag_id: Option<String>,
}

async fn update_entity(
    Path(id): Path<u32>,
    State(pool): State<Pool>,
    Json(update_entity): Json<CreateEntityDTO>,
) -> impl IntoResponse {
    let entity = entity_service::update_entity(id, update_entity.into_create_entity(), &pool)
        .await
        .unwrap();
    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::OK, Json(entity_dto))
}

async fn delete_entity(Path(id): Path<u32>, State(pool): State<Pool>) -> impl IntoResponse {
    entity_service::delete_entity(id, &pool).await.unwrap();
    StatusCode::NO_CONTENT
}

#[derive(Deserialize)]
pub struct CreateEntityDTO {
    tag_id: String,
    name: String,
    user_id: u32,
}

impl CreateEntityDTO {
    pub fn into_create_entity(self) -> CreateEntity {
        CreateEntity {
            tag_id: self.tag_id,
            name: self.name,
            user_id: self.user_id,
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityDTO {
    id: u32,
    tag_id: String,
    name: String,
    user_id: u32,
}

impl EntityDTO {
    fn from_entity(entity: Entity) -> Self {
        Self {
            id: entity.id,
            tag_id: entity.tag_id,
            name: entity.name,
            user_id: entity.user_id,
        }
    }
}
