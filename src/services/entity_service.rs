use crate::database::dao::entity_dao;

use super::errors::{ServiceError, ServiceResult};

pub async fn create_entity(entity: CreateEntity) -> anyhow::Result<u32> {
    let entity_table = entity_dao::create_entity(entity.tag_uid, entity.name, entity.user_id, entity.parent_id).await?;
    Ok(entity_table.id)
}

pub async fn get_entity(id: u32) -> ServiceResult<Entity> {
    let entity = entity_dao::get_entity(id, 1).await?;
    let entity = Entity::from_rich_entity(entity);
    Ok(entity)
}

pub async fn get_entity_closure_by_tag_uid(tag_uid: String) -> ServiceResult<EntityClosure> {
    let entity = entity_dao::get_entity_by_tag_uid(tag_uid, 1).await?;
    let entity_closure = get_entity_closure(entity.id).await?;
    Ok(entity_closure)
}

pub async fn get_entity_closures() -> ServiceResult<Vec<EntityClosure>> {
    // Get entities
    let entity_closures = entity_dao::get_entity_closures(1).await?;
    let entity_closures_mapped = entity_closures
        .into_iter()
        .map(EntityClosure::from_entity_closure)
        .collect();
    Ok(entity_closures_mapped)
}

pub async fn get_entity_closure(id: u32) -> ServiceResult<EntityClosure> {
    // Get entities
    let entity_closures = entity_dao::get_entity_closures(1).await?;
    let entity_closure = find_entity(id, entity_closures).ok_or(ServiceError::NotFound)?;

    let entity_closure_mapped = EntityClosure::from_entity_closure(entity_closure);
    Ok(entity_closure_mapped)
}

fn find_entity(id: u32, entity_closures: Vec<entity_dao::EntityClosure>) -> Option<entity_dao::EntityClosure> {
    for entity_closure in entity_closures {
        if entity_closure.id == id {
            return Some(entity_closure)
        }

        let found = find_entity(id, entity_closure.children);
        if found.is_some() {
            return found
        } 
    }
    
    return None;
}

pub async fn update_entity(id: u32, entity: CreateEntity) -> ServiceResult<()> {
    entity_dao::update_entity(id, 1, entity.tag_uid, entity.name).await?;
    Ok(())
}

pub async fn delete_entity(id: u32) -> anyhow::Result<()> {
    entity_dao::delete_entity(id, 1).await?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct CreateEntity {
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
}

pub struct Entity {
    pub id: u32,
    #[allow(dead_code)]
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
    pub parent_name: Option<String>,
}

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
}
