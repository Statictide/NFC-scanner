use crate::database::dao::entity_dao;

use super::errors::ServiceResult;

pub async fn create_entity(entity: CreateEntity) -> anyhow::Result<u32> {
    let entity_table = entity_dao::create_entity(entity.tag_uid, entity.name, entity.user_id, entity.parent_id).await?;
    Ok(entity_table.id)
}

pub async fn get_entity(id: u32) -> ServiceResult<Entity> {
    let entity = entity_dao::get_entity(id, 1).await?;
    let entity = Entity::from_rich_entity(entity);
    Ok(entity)
}

pub async fn get_entity_by_tag_uid(tag_uid: String) -> ServiceResult<Entity> {
    let entity_table = entity_dao::get_entity_by_tag_uid(tag_uid, 1).await?;
    let entity = Entity::from_rich_entity(entity_table);
    Ok(entity)
}

pub async fn get_entities(user_id: u32) -> ServiceResult<Vec<EntityClosure>> {
    // Get entities
    let entity_closures = entity_dao::get_entity_closures(user_id).await?;
    let entity_closures_mapped = entity_closures
        .into_iter()
        .map(EntityClosure::from_entity_closure)
        .collect();
    Ok(entity_closures_mapped)
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
