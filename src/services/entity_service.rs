use anyhow::bail;

use crate::database::dao::entity_dao;

use crate::services::errors::{ServiceError, ServiceResult};
use crate::util::Maybe;

// FIXME:This is a global user id, this should be a parameter in the future
const USER_ID: u32 = 1;

pub async fn create_entity(entity: CreateEntity) -> anyhow::Result<u32> {
    let entity_table = entity_dao::create_entity(USER_ID, entity.tag_uid, entity.name, entity.parent_id).await?;
    Ok(entity_table.id)
}

pub async fn get_entity_closure_by_tag_uid(tag_uid: String) -> ServiceResult<EntityClosure> {
    let entity = entity_dao::get_entity_by_tag_uid(tag_uid, USER_ID).await?;
    let entity_closure = get_entity_closure(entity.id).await?;
    Ok(entity_closure)
}

pub async fn get_entity_closures() -> ServiceResult<Vec<EntityClosure>> {
    let entities = entity_dao::get_entity_closures(USER_ID).await?;
    let entity_closures_mapped = entities
        .into_iter()
        .map(EntityClosure::from_entity_closure)
        .collect();
    Ok(entity_closures_mapped)
}

pub async fn get_entity_closure(id: u32) -> ServiceResult<EntityClosure> {
    let entities = entity_dao::get_entity_closures(USER_ID).await?;
    let entities = entities.into_iter().map(EntityClosure::from_entity_closure).collect::<Vec<_>>();
    let entity = EntityClosure::find_entity(entities, id).ok_or(ServiceError::NotFound)?;

    Ok(entity)
}

pub async fn update_entity(entity_id: u32, update: CreateEntity) -> ServiceResult<EntityClosure> {
    println!("update_entity: {:?}", update);
    let existing = get_entity_closure(entity_id).await?;
    
    // Check for circular reference
    if let Some(update_parent_id) = update.parent_id {
        // Check self assign
        let is_assigning_to_self = update_parent_id == entity_id;
        if is_assigning_to_self {
            return Err(ServiceError::CircularReference);
        }
    
        // Check circular reference
        if let Some(_) = EntityClosure::find_entity(existing.children, update_parent_id) {
            return Err(ServiceError::CircularReference);
        }
    }
    
    entity_dao::update_entity(entity_id, USER_ID, update.tag_uid, update.name, update.parent_id).await?;
    let entity_closure = get_entity_closure(entity_id).await?;
    Ok(entity_closure)
}

pub async fn patch_entity(entity_id: u32, patch_entity: PatchEntity) -> ServiceResult<EntityClosure> {
    println!("patch_entity: {:?}", patch_entity);
    let mut entity = entity_dao::get_entity(entity_id, USER_ID).await?;

    // Overwrite the fields that are present in the patch entity
    if let Some(tag_uid) = patch_entity.tag_uid {
        entity.tag_uid = tag_uid;
    };

    if let Some(name) = patch_entity.name {
        entity.name = name;
    };

    if let Some(parent_id) = patch_entity.parent_id {
        entity.parent_id = parent_id.value
    };

    let update = CreateEntity {
        tag_uid: entity.tag_uid,
        name: entity.name,
        parent_id: entity.parent_id,
    };

    update_entity(entity_id, update).await?;

    let entity_closure = get_entity_closure(entity_id).await?;
    Ok(entity_closure)
}

pub async fn delete_entity(id: u32) -> anyhow::Result<()> {
    entity_dao::delete_entity(id, USER_ID).await?;
    Ok(())
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateEntity {
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
}

#[derive(serde::Deserialize, Debug)]
pub struct PatchEntity {
    pub tag_uid: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<Maybe<u32>>,
}

#[allow(dead_code)]
pub struct Entity {
    pub id: u32,
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
    pub parent_name: Option<String>,
}

#[allow(dead_code)]
impl Entity {
    fn from_rich_entity(entity: entity_dao::EntityRich) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            tag_uid: entity.tag_uid,
            name: entity.name,
            parent_id: entity.parent_id,
            parent_name: entity.parent_name,
        }
    }
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct EntityClosure {
    pub id: u32,
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
    pub parent_name: Option<String>,
    pub children: Vec<EntityClosure>,
}

impl EntityClosure {
    fn from_entity_closure(e: entity_dao::EntityClosure) -> Self {
        Self {
            id: e.id,
            user_id: e.user_id,
            tag_uid: e.tag_uid,
            name: e.name,
            parent_id: e.parent_id,
            parent_name: e.parent_name,
            children: e.children.into_iter().map(EntityClosure::from_entity_closure).collect(),
        }
    }

    fn find_entity(entity_closures: Vec<Self>, id: u32) -> Option<Self> {
        for entity_closure in entity_closures {
            if entity_closure.id == id {
                return Some(entity_closure);
            }
    
            let found = Self::find_entity(entity_closure.children, id);
            if found.is_some() {
                return found;
            }
        }
    
        return None;
    }
}


