use std::collections::HashMap;

use crate::database::{
    db,
    errors::{DatabaseError, DatabaseResult},
};

pub async fn create_entity(
    user_id: u32,
    tag_uid: Option<String>,
    name: String,
    parent_id: Option<u32>,
) -> DatabaseResult<EntityTable> {
    let entity: EntityTable = sqlx::query_as(
        r#"
            insert into entity (user_id, name, tag_uid, parent_id) values ($1, $2, $3, $4) 
            returning *
            "#,
    )
    .bind(user_id)
    .bind(name)
    .bind(tag_uid)
    .bind(parent_id)
    .fetch_one(db::pool().await)
    .await?;

    Ok(entity)
}

pub async fn get_entity(id: u32, user_id: u32) -> DatabaseResult<EntityTable> {
    let entity = sqlx::query_as(
        r#"
            select *
            from entity e
            where e.id = $1 and e.user_id = $2
            "#,
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(db::pool().await)
    .await?;

    Ok(entity)
}

pub async fn get_entity_by_tag_uid(tag_uid: String, user_id: u32) -> DatabaseResult<EntityTable> {
    let (entity_id,): (u32,) = sqlx::query_as("select id from entity where tag_uid = $1")
        .bind(&tag_uid)
        .fetch_one(db::pool().await)
        .await?;

    let entity = get_entity(entity_id, user_id).await?;

    Ok(entity)
}

pub async fn get_entity_closures(user_id: u32) -> DatabaseResult<Vec<EntityClosure>> {
    // Get entities
    let entities = get_entities(user_id).await?;

    // Create a lookup table for the entities
    let entity_map = entities
        .into_iter()
        .map(|entity| (entity.entity_id, entity))
        .collect::<HashMap<u32, EntityRich>>();

    // Get the main entities (entities without a parent)
    let main_entities: Vec<EntityRich> = entity_map
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
                entity_id: entity.entity_id,
                user_id: entity.user_id,
                tag_uid: entity.tag_uid,
                name: entity.name,
                parent_id: entity.parent_id,
                parent_name: entity.parent_name,
                children,
            }
        })
        .collect::<Vec<_>>();

    Ok(entity_closures)
}

fn get_children_recursively(entity: &EntityRich, entity_map: &HashMap<u32, EntityRich>) -> Vec<EntityClosure> {
    let children = entity_map
        .values()
        .filter(|child| {
            let Some(parent_id) = child.parent_id else {
                return false;
            };
            return parent_id == entity.entity_id;
        })
        .cloned()
        .collect::<Vec<_>>();

    let children_closures: Vec<EntityClosure> = children
        .into_iter()
        .map(|child| {
            let children = get_children_recursively(&child, entity_map);
            EntityClosure {
                entity_id: child.entity_id,
                user_id: child.user_id,
                tag_uid: child.tag_uid,
                name: child.name,
                parent_id: child.parent_id,
                parent_name: child.parent_name,
                children,
            }
        })
        .collect();

    children_closures
}

pub async fn get_entities(user_id: u32) -> DatabaseResult<Vec<EntityRich>> {
    let entities: Vec<EntityRich> = sqlx::query_as(
        r#"
            select e.*, parent.name as parent_name
            from entity e
            left join entity parent on parent.id = e.parent_id
            where e.user_id = $1
            "#,
    )
    .bind(user_id)
    .fetch_all(db::pool().await)
    .await?;

    Ok(entities)
}

pub async fn update_entity(
    id: u32,
    user_id: u32,
    tag_uid: Option<String>,
    name: String,
    parent_id: Option<u32>,
) -> DatabaseResult<()> {
    let result =
        sqlx::query("update entity set name = $1, tag_uid = $2, parent_id = $3 where id = $4 and user_id = $5")
            .bind(name)
            .bind(tag_uid)
            .bind(parent_id)
            .bind(id)
            .bind(user_id)
            .execute(db::pool().await)
            .await?;

    if result.rows_affected() != 1 {
        return Err(DatabaseError::NotFound);
    }

    Ok(())
}

pub async fn delete_entity(id: u32, user_id: u32) -> DatabaseResult<()> {
    let result = sqlx::query("delete from entity where id = $1 and user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(db::pool().await)
        .await?;

    if result.rows_affected() != 1 {
        return Err(DatabaseError::NotFound);
    }

    Ok(())
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct EntityTable {
    #[sqlx(rename = "id")]
    pub entity_id: u32,
    #[allow(dead_code)]
    pub user_id: u32,
    pub tag_uid: Option<String>,
    pub name: String,
    pub parent_id: Option<u32>,
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct EntityRich {
    #[sqlx(rename = "id")]
    pub entity_id: u32,
    pub user_id: u32,
    pub tag_uid: Option<String>,
    pub name: String,
    pub parent_id: Option<u32>,
    pub parent_name: Option<String>,
}

#[derive(Debug)]
pub struct EntityClosure {
    pub entity_id: u32,
    pub user_id: u32,
    pub tag_uid: Option<String>,
    pub name: String,
    pub parent_id: Option<u32>,
    pub parent_name: Option<String>,
    pub children: Vec<EntityClosure>,
}
