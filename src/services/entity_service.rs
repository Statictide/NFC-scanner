use crate::database::entity_dao::{self, EntityTable};

use super::errors::ServiceResult;

pub async fn create_entity(entity: CreateEntity) -> anyhow::Result<Entity> {
    let entity_table =
        entity_dao::create_entity(entity.tag_uid, entity.name, entity.user_id, entity.parent_id).await?;

    let entity = Entity::from(entity_table);

    return Ok(entity);
}

pub async fn get_entity(id: u32) -> ServiceResult<EntityClosure> {
    let entity_table = entity_dao::get_entity(id).await?;
    let entity = EntityClosure::from(entity_table);
    Ok(entity)
}

pub async fn get_entity_by_tag_id(tag_uid: String) -> ServiceResult<EntityClosure> {
    let entity_table = entity_dao::get_entity_by_tag_uid(tag_uid).await?;
    let entity = EntityClosure::from(entity_table);
    Ok(entity)
}

pub async fn get_entities_by_user_id(user_id: u32) -> ServiceResult<Vec<Entity>> {
    let entities_table = entity_dao::get_entities_by_user_id(user_id).await?;
    let entities = entities_table.into_iter().map(Entity::from).collect();
    return Ok(entities);
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

pub struct EntityClosure {
    pub entity: Entity,
    pub parent: Option<Entity>,
    pub children: Vec<Entity>,
}

impl From<entity_dao::EntityClosure> for EntityClosure {
    fn from(entity: entity_dao::EntityClosure) -> Self {
        Self {
            entity: Entity::from(entity.entity),
            parent: entity.parent.map(Entity::from),
            children: entity.children.into_iter().map(Entity::from).collect(),
        }
    }
}
