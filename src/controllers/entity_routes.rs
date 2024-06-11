use crate::services::entity_service::{self, CreateEntity, Entity};

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

pub fn get_entity_routes() -> Router {
    Router::new()
        .route("/", post(create_entity).get(get_entities))
        .route(
            "/:id",
            get(get_entity).put(update_entity).delete(delete_entity),
        )
}

async fn create_entity(Json(create_entity): Json<CreateEntityDTO>) -> Result<Response, Response> {
    let entity = entity_service::create_entity(create_entity.into_create_entity())
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;

    let entity_dto = EntityDTO::from_entity(entity);
    let r = (StatusCode::CREATED, Json(entity_dto)).into_response();
    Ok(r)
}

async fn get_entity(Path(id): Path<u32>) -> impl IntoResponse {
    let entity = entity_service::get_entity(id).await.unwrap();
    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::OK, Json(entity_dto))
}

async fn get_entities(Query(query): Query<TagIdQuery>) -> impl IntoResponse {
    if let Some(tag_id) = query.tag_id {
        let entity_option = entity_service::get_entity_by_tag_id(tag_id).await.unwrap();

        let Some(entity) = entity_option else {
            return (StatusCode::OK, Json(vec![]));
        };

        let entity_dto = EntityDTO::from_entity(entity);
        return (StatusCode::OK, Json(vec![entity_dto]));
    }

    let entities = entity_service::get_entities().await.unwrap();
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
    Json(update_entity): Json<CreateEntityDTO>,
) -> impl IntoResponse {
    let entity = entity_service::update_entity(id, update_entity.into_create_entity())
        .await
        .unwrap();
    let entity_dto = EntityDTO::from_entity(entity);
    (StatusCode::OK, Json(entity_dto))
}

async fn delete_entity(Path(id): Path<u32>) -> impl IntoResponse {
    entity_service::delete_entity(id).await.unwrap();
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
