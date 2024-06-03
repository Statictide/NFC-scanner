use crate::database::entity_dao;

pub async fn create_entity(
    entity: CreateEntity,
    pool: &sqlx::SqlitePool,
) -> anyhow::Result<Entity> {
    let entity_table: entity_dao::EntityTable =
        entity_dao::create_entity(entity.tag_id, entity.name, entity.owner, pool).await?;

    return Ok(Entity::from_entity_table(entity_table));
}

pub async fn get_entity(id: u32, pool: &sqlx::SqlitePool) -> anyhow::Result<Entity> {
    let entity_table = entity_dao::get_entity(id, pool).await?;
    let entity = Entity::from_entity_table(entity_table);
    Ok(entity)
}

pub async fn get_entities(pool: &sqlx::SqlitePool) -> anyhow::Result<Vec<Entity>> {
    let entities_table = entity_dao::get_entities(pool).await?;
    let entities = entities_table
        .into_iter()
        .map(Entity::from_entity_table)
        .collect();
    Ok(entities)
}

pub async fn update_entity(
    id: u32,
    entity: CreateEntity,
    pool: &sqlx::Pool<sqlx::Sqlite>,
) -> anyhow::Result<Entity> {
    let entity_table =
        entity_dao::update_entity(id, entity.tag_id, entity.name, entity.owner, pool).await?;

    return Ok(Entity::from_entity_table(entity_table));
}

pub async fn delete_entity(id: u32, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    entity_dao::delete_entity(id, pool).await?;
    return Ok(());
}

#[derive(serde::Deserialize)]
pub struct CreateEntity {
    pub tag_id: String,
    pub name: String,
    pub owner: String,
}

pub struct Entity {
    pub id: u32,
    pub tag_id: String,
    pub name: String,
    pub owner: String,
}

impl Entity {
    pub fn from_entity_table(entity: entity_dao::EntityTable) -> Self {
        Self {
            id: entity.id,
            tag_id: entity.tag_id,
            name: entity.name,
            owner: entity.owner,
        }
    }
}
