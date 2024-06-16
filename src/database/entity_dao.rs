use std::collections::HashMap;

use anyhow::bail;

use super::db;

pub async fn create_entity(
    tag_uid: String,
    name: String,
    user_id: u32,
    parrent_id: Option<u32>,
) -> sqlx::Result<EntityTable> {
    let entity: EntityTable = sqlx::query_as(
        "insert into entity (name, tag_uid, user_id, parrent_id) values ($1, $2, $3, $4) returning *",
    )
    .bind(name)
    .bind(tag_uid)
    .bind(user_id)
    .bind(parrent_id)
    .fetch_one(db::get_db().await)
    .await?;

    Ok(entity)
}

pub async fn get_entity(id: u32) -> anyhow::Result<EntityClosure> {
    // Get the entity with the given id, the parrent of the entity, and all children of the entity
    let entities: Vec<EntityTable> = sqlx::query_as(
        r#"
        with main_entity_parrent_id as (
            select parrent_id from entity where id = $1
        )
        select * 
        from entity 
        where 
            id = $1 
            or id = (select parrent_id from main_entity_parrent_id)
            or parrent_id = $1 
    "#,
    )
    .bind(id)
    .fetch_all(db::get_db().await)
    .await?;

    // Create a lookup table for the entities
    let entity_map = entities
        .into_iter()
        .map(|entity| (entity.id, entity))
        .collect::<HashMap<u32, EntityTable>>();

    // Get the main entity, parrent entity, and children entities
    let Some(main_entity) = entity_map.get(&id).cloned() else {
        bail!("Entity with id={id} not found")
    };

    let parrent_entity = main_entity
        .parrent_id
        .map(|id| entity_map.get(&id))
        .unwrap_or_default()
        .cloned();

    let child_entities: Vec<EntityTable> = entity_map
        .into_values()
        .filter(|entity| {
            let Some(parrent_id) = entity.parrent_id else {
                return false;
            };
            return parrent_id == main_entity.id;
        })
        .collect::<Vec<_>>();

    let entity_closure = EntityClosure {
        entity: main_entity,
        parrent: parrent_entity,
        children: child_entities,
    };

    Ok(entity_closure)
}

pub async fn get_all_entities_by_user_id(user_id: u32) -> anyhow::Result<Vec<EntityTable>> {
    let entities: Vec<EntityTable> = sqlx::query_as("select * from entity where user_id = $1")
        .bind(user_id)
        .fetch_all(db::get_db().await)
        .await?;

    Ok(entities)
}

pub async fn update_entity(
    id: u32,
    tag_uid: String,
    name: String,
    user_id: u32,
) -> sqlx::Result<()> {
    sqlx::query_as("update entity set name = $1, tag_uid = $2, user_id = $3 where id = $4")
        .bind(name)
        .bind(tag_uid)
        .bind(user_id)
        .bind(id)
        .fetch_one(db::get_db().await)
        .await?;

    Ok(())
}

pub async fn delete_entity(id: u32) -> anyhow::Result<()> {
    sqlx::query("delete from entity where id = $1")
        .bind(id)
        .execute(db::get_db().await)
        .await?;

    Ok(())
}

pub async fn get_entity_by_tag_uid(tag_uid: String) -> anyhow::Result<EntityTable> {
    let entity: EntityTable = sqlx::query_as("select * from entity where tag_uid = $1")
        .bind(&tag_uid)
        .fetch_one(db::get_db().await)
        .await?;

    Ok(entity)
}

#[derive(serde::Serialize, sqlx::FromRow, Clone)]
pub struct EntityTable {
    pub id: u32,
    pub user_id: u32,
    pub tag_uid: String,
    pub name: String,
    pub parrent_id: Option<u32>,
}

pub struct EntityClosure {
    pub entity: EntityTable,
    pub parrent: Option<EntityTable>,
    pub children: Vec<EntityTable>,
}
