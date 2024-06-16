use crate::database::entity_dao::{self, EntityTable};

pub async fn create_entity(entity: CreateEntity) -> anyhow::Result<Entity> {
    let entity_table = entity_dao::create_entity(
        entity.tag_uid,
        entity.name,
        entity.user_id,
        entity.parrent_id,
    )
    .await?;

    let entity = Entity::from(entity_table);

    return Ok(entity);
}

pub async fn get_entity(id: u32) -> anyhow::Result<EntityClosure> {
    let entity_table = entity_dao::get_entity(id).await?;
    let entity = EntityClosure::from(entity_table);
    Ok(entity)
}

pub async fn get_entities_by_user_id(user_id: u32) -> anyhow::Result<Vec<Entity>> {
    let entities_table = entity_dao::get_all_entities_by_user_id(user_id).await?;
    let entities = entities_table.into_iter().map(Entity::from).collect();
    return Ok(entities);
}

pub async fn update_entity(id: u32, entity: CreateEntity) -> anyhow::Result<()> {
    entity_dao::update_entity(id, entity.tag_uid, entity.name, entity.user_id).await?;

    return Ok(());
}

pub async fn delete_entity(id: u32) -> anyhow::Result<()> {
    entity_dao::delete_entity(id).await?;
    return Ok(());
}

pub async fn get_entity_by_tag_id(tag_id: String) -> anyhow::Result<Entity> {
    let entity_table = entity_dao::get_entity_by_tag_uid(tag_id).await?;
    let entity = Entity::from(entity_table);
    Ok(entity)
}

#[derive(serde::Deserialize)]
pub struct CreateEntity {
    pub user_id: u32,
    pub name: String,
    pub tag_uid: String,
    pub parrent_id: Option<u32>,
}

pub struct Entity {
    pub id: u32,
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parrent_id: Option<u32>,
}

impl From<EntityTable> for Entity {
    fn from(entity: entity_dao::EntityTable) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            tag_uid: entity.tag_uid,
            name: entity.name,
            parrent_id: entity.parrent_id,
        }
    }
}

pub struct EntityClosure {
    pub entity: Entity,
    pub parrent: Option<Entity>,
    pub children: Vec<Entity>,
}

impl From<entity_dao::EntityClosure> for EntityClosure {
    fn from(entity: entity_dao::EntityClosure) -> Self {
        Self {
            entity: Entity::from(entity.entity),
            parrent: entity.parrent.map(Entity::from),
            children: entity.children.into_iter().map(Entity::from).collect(),
        }
    }
}
