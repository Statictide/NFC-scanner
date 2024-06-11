use crate::database::entity_dao;

pub async fn create_entity(entity: CreateEntity) -> anyhow::Result<Entity> {
    let entity_table: entity_dao::EntityTable =
        entity_dao::create_entity(entity.tag_id, Some(entity.name), Some(entity.user_id)).await?;

    return Ok(Entity::from_entity_table(entity_table));
}

pub async fn get_entity(id: u32) -> anyhow::Result<Entity> {
    let entity_table = entity_dao::get_entity(id).await?;
    let entity = Entity::from_entity_table(entity_table);
    Ok(entity)
}

pub async fn get_entities() -> anyhow::Result<Vec<Entity>> {
    let entities_table = entity_dao::get_entities().await?;
    let entities = entities_table
        .into_iter()
        .map(Entity::from_entity_table)
        .collect();
    Ok(entities)
}

pub async fn update_entity(id: u32, entity: CreateEntity) -> anyhow::Result<Entity> {
    let entity_table =
        entity_dao::update_entity(id, entity.tag_id, entity.name, entity.user_id).await?;

    return Ok(Entity::from_entity_table(entity_table));
}

pub async fn delete_entity(id: u32) -> anyhow::Result<()> {
    entity_dao::delete_entity(id).await?;
    return Ok(());
}

pub async fn get_entity_by_tag_id(tag_id: String) -> anyhow::Result<Option<Entity>> {
    let entity_table = entity_dao::get_entity_by_tag_id(tag_id).await?;
    let entity = entity_table.map(Entity::from_entity_table);
    Ok(entity)
}

#[derive(serde::Deserialize)]
pub struct CreateEntity {
    pub tag_id: String,
    pub name: String,
    pub user_id: u32,
}

pub struct Entity {
    pub id: u32,
    pub tag_id: String,
    pub name: String,
    pub user_id: u32,
}

impl Entity {
    pub fn from_entity_table(entity: entity_dao::EntityTable) -> Self {
        Self {
            id: entity.id,
            tag_id: entity.tag_id,
            name: entity.name,
            user_id: entity.user_id,
        }
    }
}
