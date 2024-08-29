use std::collections::HashMap;

use crate::database::{
    db,
    errors::{DatabaseError, DatabaseResult},
};

pub async fn create_entity(
    tag_uid: String,
    name: String,
    user_id: u32,
    parent_id: Option<u32>,
) -> DatabaseResult<EntityTable> {
    let entity: EntityTable =
        sqlx::query_as("insert into entity (name, tag_uid, user_id, parent_id) values ($1, $2, $3, $4) returning *")
            .bind(name)
            .bind(tag_uid)
            .bind(user_id)
            .bind(parent_id)
            .fetch_one(db::pool().await)
            .await?;

    Ok(entity)
}

pub async fn _get_entity(id: u32) -> DatabaseResult<EntityTable> {
    let entity: EntityTable = sqlx::query_as("select * from entity where id = $1")
        .bind(id)
        .fetch_one(db::pool().await)
        .await?;

    Ok(entity)
}

pub async fn get_entity_enriched(id: u32) -> DatabaseResult<EnrichedEntity> {
    // Get the entity with the given id, the parent of the entity, and all children of the entity
    let entities: Vec<EntityTable> = sqlx::query_as(
        r#"
        with main_entity_parent_id as (
            select parent_id from entity where id = $1
        )
        select * 
        from entity 
        where 
            id = $1 
            or id = (select parent_id from main_entity_parent_id)
            or parent_id = $1 
    "#,
    )
    .bind(id)
    .fetch_all(db::pool().await)
    .await?;

    // Create a lookup table for the entities
    let entity_map = entities
        .into_iter()
        .map(|entity| (entity.id, entity))
        .collect::<HashMap<u32, EntityTable>>();

    // Get the main entity, parent entity, and children entities
    let Some(main_entity) = entity_map.get(&id).cloned() else {
        return Err(DatabaseError::NotFound);
    };

    let parent_entity = main_entity
        .parent_id
        .map(|id| entity_map.get(&id))
        .unwrap_or_default()
        .cloned();

    let child_entities: Vec<EntityTable> = entity_map
        .into_values()
        .filter(|entity| {
            let Some(parent_id) = entity.parent_id else {
                return false;
            };
            return parent_id == main_entity.id;
        })
        .collect::<Vec<_>>();

    let entity_closure = EnrichedEntity {
        entity: main_entity,
        parent: parent_entity,
        children: child_entities,
    };

    Ok(entity_closure)
}

pub async fn get_entities_by_user_id(user_id: u32) -> DatabaseResult<Vec<EntityTable>> {
    let entities: Vec<EntityTable> = sqlx::query_as("select * from entity where user_id = $1")
        .bind(user_id)
        .fetch_all(db::pool().await)
        .await?;

    Ok(entities)
}

pub async fn update_entity(id: u32, user_id: u32, tag_uid: String, name: String) -> DatabaseResult<()> {
    sqlx::query_as("update entity set name = $1, tag_uid = $2, user_id = $3 where id = $4")
        .bind(name)
        .bind(tag_uid)
        .bind(user_id)
        .bind(id)
        .fetch_one(db::pool().await)
        .await?;

    Ok(())
}

pub async fn delete_entity(id: u32) -> DatabaseResult<()> {
    sqlx::query("delete from entity where id = $1")
        .bind(id)
        .execute(db::pool().await)
        .await?;

    Ok(())
}

pub async fn get_entity_by_tag_uid(tag_uid: String) -> DatabaseResult<EnrichedEntity> {
    let (entity_id,): (u32,) = sqlx::query_as("select id from entity where tag_uid = $1")
        .bind(&tag_uid)
        .fetch_one(db::pool().await)
        .await?;

    let entity = get_entity_enriched(entity_id).await?;

    Ok(entity)
}

#[derive(serde::Serialize, sqlx::FromRow, Clone, Debug)]
pub struct EntityTable {
    pub id: u32,
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parent_id: Option<u32>,
}

#[derive(Debug)]
pub struct EnrichedEntity {
    pub entity: EntityTable,
    pub parent: Option<EntityTable>,
    pub children: Vec<EntityTable>,
}
