use crate::services::{entity_service, errors::ServiceError};

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
                .patch(patch_entity)
                .delete(delete_entity),
        )
        .route("/by-tag", get(get_entity_by_tag))
}

async fn create_entity(Json(create_entity): Json<CreateEntityDTO>) -> AppResult<impl IntoResponse> {
    let id = entity_service::create_entity(create_entity.0).await?;

    Ok((StatusCode::CREATED, Json(id)))
}

async fn get_entity(Path(id): Path<u32>) -> AppResult<impl IntoResponse> {
    let entity_closure = entity_service::get_entity_closure(id).await?;
    let entity_closure: EntityClosureDTO = EntityClosureDTO::from_entity_closure(entity_closure);
    Ok((StatusCode::OK, Json(entity_closure)))
}

#[derive(Deserialize)]
struct TagIdQuery {
    pub tag_uid: String,
    pub create: Option<bool>,
}

async fn get_entity_by_tag(Query(TagIdQuery { tag_uid, create }): Query<TagIdQuery>) -> AppResult<impl IntoResponse> {
    let entity_result = entity_service::get_entity_closure_by_tag_uid(tag_uid.clone()).await;

    let create = create.unwrap_or(false);
    let entity = match (entity_result, create) {
        // Found entity
        (Ok(entity), _) => entity,

        // Not found, create
        (Err(ServiceError::NotFound), true) => {
            let create_entity = entity_service::CreateEntity {
                tag_uid: tag_uid,
                name: String::new(),
                parent_id: None,
            };
            let id = entity_service::create_entity(create_entity).await?;
            entity_service::get_entity_closure(id).await?
        }

        // Error
        (Err(err), _) => return Err(err.into()),
    };

    let entity = EntityClosureDTO::from_entity_closure(entity);

    return Ok((StatusCode::OK, Json(entity)));
}

#[derive(Deserialize)]
struct UserIdQuery {
    #[allow(dead_code)]
    pub user_id: u32,
}

async fn get_entities(Query(_): Query<UserIdQuery>) -> AppResult<impl IntoResponse> {
    let entities = entity_service::get_entity_closures().await?;
    let mut entities_dto: Vec<EntityClosureDTO> = entities
        .into_iter()
        .map(EntityClosureDTO::from_entity_closure)
        .collect::<Vec<_>>();
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
    let entity_closure = entity_service::update_entity(id, update_entity.0).await?;

    Ok((StatusCode::OK, Json(entity_closure)))
}

async fn patch_entity(Path(id): Path<u32>, Json(patch_entity): Json<PatchEntityDTO>) -> AppResult<impl IntoResponse> {
    let entity_closure = entity_service::patch_entity(id, patch_entity.0).await?;

    Ok((StatusCode::OK, Json(entity_closure)))
}

async fn delete_entity(Path(id): Path<u32>) -> AppResult<impl IntoResponse> {
    entity_service::delete_entity(id).await?;
    return Ok(StatusCode::NO_CONTENT);
}

#[derive(Deserialize, Debug)]
pub struct CreateEntityDTO(entity_service::CreateEntity);

#[derive(Deserialize, Debug)]
pub struct PatchEntityDTO(entity_service::PatchEntity);

#[derive(serde::Serialize)]
pub struct EntityClosureDTO {
    pub id: u32,
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
    pub parent_name: Option<String>,
    pub children: Vec<EntityClosureDTO>,
}

impl EntityClosureDTO {
    fn from_entity_closure(e: entity_service::EntityClosure) -> Self {
        Self {
            id: e.id,
            user_id: e.user_id,
            tag_uid: e.tag_uid,
            name: e.name,
            parent_id: e.parent_id,
            parent_name: e.parent_name,
            children: e
                .children
                .into_iter()
                .map(EntityClosureDTO::from_entity_closure)
                .collect(),
        }
    }
}
