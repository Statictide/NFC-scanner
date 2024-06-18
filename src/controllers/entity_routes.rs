use crate::{controllers::errors::AppError, services::entity_service};

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
            get(get_entity)
                .put(update_entity)
                .patch(assign_new_parent_id_and_get_parent_entity)
                .delete(delete_entity),
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

    return Ok((StatusCode::OK, Json(entity_closure_dto)));
}

#[derive(Deserialize)]
struct TagIdQuery {
    pub tag_uid: String,
}

async fn get_entity_by_tag(Query(TagIdQuery { tag_uid }): Query<TagIdQuery>) -> AppResult<impl IntoResponse> {
    let entity = entity_service::get_entity_by_tag_id(tag_uid).await?;
    let entity = EntityClosureDTO::from(entity);

    return Ok((StatusCode::OK, Json(entity)));
}

#[derive(Deserialize)]
struct UserIdQuery {
    pub user_id: u32,
}

async fn get_entities(Query(UserIdQuery { user_id: _user_id }): Query<UserIdQuery>) -> AppResult<impl IntoResponse> {
    let entities = entity_service::get_entities_by_user_id(1).await?; // Fixme: user_id
    let entities_dto: Vec<_> = entities.into_iter().map(EntityDTO::from).collect();

    return Ok((StatusCode::OK, Json(entities_dto)));
}

async fn update_entity(
    Path(id): Path<u32>,
    Json(update_entity): Json<CreateEntityDTO>,
) -> AppResult<impl IntoResponse> {
    entity_service::update_entity(id, update_entity.into()).await?;

    return Ok(StatusCode::NO_CONTENT);
}

#[derive(Deserialize)]
pub struct PatchEntityDTO {
    parent_id: Option<u32>,
}

async fn assign_new_parent_id_and_get_parent_entity(
    Path(id): Path<u32>,
    Json(entity_update): Json<PatchEntityDTO>,
) -> AppResult<impl IntoResponse> {
    let Some(parent_id) = entity_update.parent_id else {
        return Err(AppError::BadRequest("Parent id is required".into()));
    };

    entity_service::update_entity_partial(id, parent_id).await?;
    let parent_entity = entity_service::get_entity(parent_id).await?;
    let parent_entity = EntityClosureDTO::from(parent_entity);

    return Ok((StatusCode::OK, Json(parent_entity)));
}

async fn delete_entity(Path(id): Path<u32>) -> AppResult<impl IntoResponse> {
    entity_service::delete_entity(id).await?;
    return Ok(StatusCode::NO_CONTENT);
}

#[derive(Deserialize)]
pub struct CreateEntityDTO {
    tag_uid: String,
    name: String,
    parent_id: Option<u32>,
}

impl Into<entity_service::CreateEntity> for CreateEntityDTO {
    fn into(self) -> entity_service::CreateEntity {
        entity_service::CreateEntity {
            user_id: 1, // Fixme: user_id
            tag_uid: self.tag_uid,
            name: self.name,
            parent_id: self.parent_id,
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityDTO {
    id: u32,
    // user_id: u32, Fixme: user_id
    tag_uid: String,
    name: String,
    parent_id: Option<u32>,
}

impl From<entity_service::Entity> for EntityDTO {
    fn from(e: entity_service::Entity) -> Self {
        Self {
            id: e.id,
            tag_uid: e.tag_uid,
            name: e.name,
            parent_id: e.parent_id,
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityClosureDTO {
    entity: EntityDTO,
    parent: Option<EntityDTO>,
    children: Vec<EntityDTO>,
}

impl From<entity_service::EntityClosure> for EntityClosureDTO {
    fn from(e: entity_service::EntityClosure) -> Self {
        Self {
            entity: e.entity.into(),
            parent: e.parent.map(EntityDTO::from),
            children: e.children.into_iter().map(EntityDTO::from).collect(),
        }
    }
}
