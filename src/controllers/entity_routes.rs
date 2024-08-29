use crate::services::entity_service;

use anyhow::anyhow;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;

use crate::controllers::errors::AppResult;

use super::errors::AppError;

pub fn get_entity_routes() -> Router {
    Router::new()
        .route("/", post(create_entity).get(get_entities))
        .route("/:id", put(update_entity).delete(delete_entity))
        .route("/by-tag", get(get_entity_by_tag))
}

async fn create_entity(Json(create_entity): Json<CreateEntityDTO>) -> AppResult<impl IntoResponse> {
    let id = entity_service::create_entity(create_entity.into()).await?;

    Ok((StatusCode::CREATED, Json(id)))
}

#[derive(Deserialize)]
struct TagIdQuery {
    pub tag_uid: String,
}

async fn get_entity_by_tag(Query(TagIdQuery { tag_uid }): Query<TagIdQuery>) -> AppResult<impl IntoResponse> {
    let entity = entity_service::get_entity_by_tag_uid(tag_uid).await?;
    let entity = RichEntityDTO::from(entity);

    return Ok((StatusCode::OK, Json(entity)));
}

#[derive(Deserialize)]
struct UserIdQuery {
    pub user_id: u32,
}

async fn get_entities(Query(UserIdQuery { user_id: _user_id }): Query<UserIdQuery>) -> AppResult<impl IntoResponse> {
    let user_id = 1;
    tracing::info!("Using fixed user_id = {user_id}");
    let entities = entity_service::get_entities(user_id).await?;
    let mut entities_dto: Vec<EntityClosureDTO> = entities.into_iter().map(EntityClosureDTO::from_entity_closure).collect::<Vec<_>>();
    sort_entities(&mut entities_dto);

    return Ok((StatusCode::OK, Json(entities_dto)));
}

// sort entityies by id, and recursivly sort the children
fn sort_entities(entities: &mut Vec<EntityClosureDTO>) {
    entities.sort_by_key(|e| e.id);
    for entity in entities.iter_mut() {
        sort_entities(&mut entity.children);
    }
}

async fn update_entity(
    Path(id): Path<u32>,
    Json(update_entity): Json<CreateEntityDTO>,
) -> AppResult<impl IntoResponse> {
    entity_service::update_entity(id, update_entity.into()).await?;

    return Ok(StatusCode::NO_CONTENT);
}

async fn delete_entity(Path(id): Path<u32>) -> AppResult<impl IntoResponse> {
    entity_service::delete_entity(id).await?;
    return Ok(StatusCode::NO_CONTENT);
}

// TODO: Implement this route
async fn get_entity_closure_with_parent(Path(id): Path<u32>) -> AppResult<impl IntoResponse> {
    let entities = entity_service::get_entities(1).await?;
    let (entity_closure, parent) = find_entity_closure_and_parent_by_id(id, &entities);
    if entity_closure.is_none() {
        return Err(AppError::NotFound.into());
    }

    let parent = ;

    // recursively search the children for the id


    Ok(String::from("Not implemented yet"))
}

fn find_entity_closure_and_parent_by_id(id: u32, entities: &Vec<entity_service::EntityClosure>) -> (Option<&entity_service::EntityClosure>, Option<&entity_service::EntityClosure>) {
    for entity in entities {
        if entity.id == id {
            return (Some(entity), None);
        }

        // Check if the entity is in the children
        let (child, parent) = find_entity_closure_and_parent_by_id(id, &entity.children);
        let (child, parent) = 
        match (child, parent) {
            // Someone else is the parent
            (Some(child), None) => return (Some(child), Some(entity)),
            // We are the parent!
            (Some(child), Some(parent)) => return (Some(child), Some(parent)),
            // Not found
            _ => (None, None),
        };
        
        // Stop searching if we found the child
        if child.is_some() {
            return (child, parent);
        }
    }
    (None, None)
}

#[derive(Deserialize)]
pub struct CreateEntityDTO {
    tag_uid: String,
    name: String,
    parent_id: Option<u32>,
}

impl Into<entity_service::CreateEntity> for CreateEntityDTO {
    fn into(self) -> entity_service::CreateEntity {
        let user_id = 1;
        tracing::warn!("Using fixed user_id = {user_id}");
        entity_service::CreateEntity {
            user_id,
            tag_uid: self.tag_uid,
            name: self.name,
            parent_id: self.parent_id,
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityDTO {
    id: u32,
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
pub struct RichEntityDTO {
    entity: EntityDTO,
    parent: Option<EntityDTO>,
    children: Vec<EntityDTO>,
}

impl From<entity_service::EnrichedEntity> for RichEntityDTO {
    fn from(e: entity_service::EnrichedEntity) -> Self {
        Self {
            entity: EntityDTO::from(e.entity),
            parent: e.parent.map(EntityDTO::from),
            children: e.children.into_iter().map(EntityDTO::from).collect(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct EntityClosureDTO {
    pub id: u32,
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub children: Vec<EntityClosureDTO>,
}

impl EntityClosureDTO {
    fn from_entity_closure(e: entity_service::EntityClosure) -> Self {
        Self {
            id: e.id,
            user_id: e.user_id,
            tag_uid: e.tag_uid,
            name: e.name,
            children: e.children.into_iter().map(EntityClosureDTO::from_entity_closure).collect(),
        }
    }
}
