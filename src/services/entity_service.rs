use std::collections::HashMap;

use crate::database::dao::entity_dao::{self, EntityTable};

use super::errors::ServiceResult;

pub async fn create_entity(entity: CreateEntity) -> anyhow::Result<Entity> {
    let entity_table = entity_dao::create_entity(entity.tag_uid, entity.name, entity.user_id, entity.parent_id).await?;

    let entity = Entity::from(entity_table);

    return Ok(entity);
}

pub async fn get_entity(id: u32) -> ServiceResult<EnrichedEntity> {
    let entity_table = entity_dao::get_entity(id).await?;
    let entity = EnrichedEntity::from(entity_table);
    Ok(entity)
}

pub async fn get_entity_by_tag_id(tag_uid: String) -> ServiceResult<EnrichedEntity> {
    let entity_table = entity_dao::get_entity_by_tag_uid(tag_uid).await?;
    let entity = EnrichedEntity::from(entity_table);
    Ok(entity)
}

pub async fn get_entities_by_user_id(user_id: u32) -> ServiceResult<Vec<EntityClosure>> {
    // Get entities
    let entities = entity_dao::get_entities_by_user_id(user_id).await?;

    // Create a lookup table for the entities
    let entity_map = entities
        .into_iter()
        .map(|entity| (entity.id, entity))
        .collect::<HashMap<u32, EntityTable>>();

    // Get the main entities (entities without a parent)
    let main_entities: Vec<EntityTable> = entity_map
        .values()
        .filter(|entity| entity.parent_id.is_none())
        .cloned()
        .collect::<Vec<_>>();

    // Get the children entities
    let entity_closures: Vec<EntityClosure> = main_entities
        .into_iter()
        .map(|entity| {
            let children = get_children_recursively(&entity, &entity_map);
            EntityClosure {
                id: entity.id,
                user_id: entity.user_id,
                tag_uid: entity.tag_uid,
                name: entity.name,
                parent_id: entity.parent_id,
                children,
            }
        })
        .collect();

    Ok(entity_closures)
}

fn get_children_recursively(entity: &EntityTable, entity_map: &HashMap<u32, EntityTable>) -> Vec<EntityClosure> {
    let children: Vec<EntityTable> = entity_map
        .values()
        .filter(|child| {
            let Some(parent_id) = child.parent_id else {
                return false;
            };
            return parent_id == entity.id;
        })
        .cloned()
        .collect::<Vec<_>>();

    let children_closures: Vec<EntityClosure> = children
        .into_iter()
        .map(|child| {
            let children = get_children_recursively(&child, entity_map);
            EntityClosure {
                id: child.id,
                user_id: child.user_id,
                tag_uid: child.tag_uid,
                name: child.name,
                parent_id: child.parent_id,
                children,
            }
        })
        .collect();

    children_closures
}

pub async fn update_entity(id: u32, entity: CreateEntity) -> ServiceResult<()> {
    entity_dao::update_entity(id, entity.user_id, entity.tag_uid, entity.name).await?;

    return Ok(());
}

pub async fn update_entity_partial(id: u32, parent_id: u32) -> ServiceResult<()> {
    entity_dao::update_entity_parent(id, parent_id).await?;
    Ok(())
}

pub async fn delete_entity(id: u32) -> anyhow::Result<()> {
    entity_dao::delete_entity(id).await?;
    return Ok(());
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
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
}

impl From<EntityTable> for Entity {
    fn from(entity: entity_dao::EntityTable) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            tag_uid: entity.tag_uid,
            name: entity.name,
            parent_id: entity.parent_id,
        }
    }
}

pub struct EnrichedEntity {
    pub entity: Entity,
    pub parent: Option<Entity>,
    pub children: Vec<Entity>,
}

impl From<entity_dao::EnrichedEntity> for EnrichedEntity {
    fn from(entity: entity_dao::EnrichedEntity) -> Self {
        Self {
            entity: Entity::from(entity.entity),
            parent: entity.parent.map(Entity::from),
            children: entity.children.into_iter().map(Entity::from).collect(),
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
    pub children: Vec<EntityClosure>,
}
