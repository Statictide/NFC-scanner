use crate::services::entity_service;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::controllers::errors::AppResult;

pub fn get_entity_routes() -> Router {
    Router::new()
        .route("/", post(create_entity).get(get_entities))
        .route(
            "/:id",
            get(get_entity).put(update_entity).delete(delete_entity),
        )
        .route("/by-tag", get(get_entity_by_tag))
}

async fn create_entity(Json(create_entity): Json<CreateEntityDTO>) -> AppResult<impl IntoResponse> {
    let entity = entity_service::create_entity(create_entity.into()).await?;
    let entity_dto = EntityDTO::from(entity);

    Ok((StatusCode::CREATED, Json(entity_dto)))
}

async fn get_entity(Path(id): Path<u32>) -> AppResult<impl IntoResponse> {
    let entity = entity_service::get_entity(id).await?;
    let entity_closure_dto = EntityClosureDTO::from(entity);

    Ok((StatusCode::OK, Json(entity_closure_dto)))
}

#[derive(Deserialize)]
struct TagIdQuery {
    pub tag_uid: String,
}

async fn get_entity_by_tag(
    Query(TagIdQuery { tag_uid: tag_serial_number }): Query<TagIdQuery>,
) -> AppResult<impl IntoResponse> {
    let entity = entity_service::get_entity_by_tag_id(tag_serial_number).await?;
    let entity = EntityClosureDTO::from(entity);

    return Ok((StatusCode::OK, Json(entity)));
}

#[derive(Deserialize)]
struct UserIdQuery {
    pub user_id: u32,
}

async fn get_entities(
    Query(UserIdQuery { user_id }): Query<UserIdQuery>,
) -> AppResult<impl IntoResponse> {
    let entities = entity_service::get_entities_by_user_id(user_id).await?;
    let entities_dto: Vec<_> = entities.into_iter().map(EntityDTO::from).collect();

    Ok((StatusCode::OK, Json(entities_dto)))
}

async fn update_entity(
    Path(id): Path<u32>,
    Json(update_entity): Json<CreateEntityDTO>,
) -> impl IntoResponse {
    entity_service::update_entity(id, update_entity.into())
        .await
        .unwrap();
    StatusCode::NO_CONTENT
}

async fn delete_entity(Path(id): Path<u32>) -> impl IntoResponse {
    entity_service::delete_entity(id).await.unwrap();
    StatusCode::NO_CONTENT
}

#[derive(Deserialize)]
pub struct CreateEntityDTO {
    tag_uid: String,
    name: String,
    parrent: Option<u32>,
}

impl Into<entity_service::CreateEntity> for CreateEntityDTO {
    fn into(self) -> entity_service::CreateEntity {
        entity_service::CreateEntity {
            user_id: 1, // TODO: get user_id from request JWT
            tag_uid: self.tag_uid,
            name: self.name,
            parrent_id: self.parrent,
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityDTO {
    id: u32,
    tag_uid: String,
    name: String,
    parrent_id: Option<u32>,
}

impl From<entity_service::Entity> for EntityDTO {
    fn from(e: entity_service::Entity) -> Self {
        Self {
            id: e.id,
            tag_uid: e.tag_uid,
            name: e.name,
            parrent_id: e.parrent_id,
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityClosureDTO {
    entity: EntityDTO,
    parrent: Option<EntityDTO>,
    children: Vec<EntityDTO>,
}

impl From<entity_service::EntityClosure> for EntityClosureDTO {
    fn from(e: entity_service::EntityClosure) -> Self {
        Self {
            entity: e.entity.into(),
            parrent: e.parrent.map(EntityDTO::from),
            children: e.children.into_iter().map(EntityDTO::from).collect(),
        }
    }
}
