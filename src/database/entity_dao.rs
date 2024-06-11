use super::db;

pub async fn create_entity(
    tag_id: String,
    name: Option<String>,
    user_id: Option<u32>,
) -> sqlx::Result<EntityTable> {
    let entity: EntityTable = sqlx::query_as(
        "insert into entity (name, tag_id, user_id) values ($1, $2, $3) returning *",
    )
    .bind(name)
    .bind(tag_id)
    .bind(user_id)
    .fetch_one(db::get_db().await)
    .await?;

    Ok(entity)
}

pub async fn get_entity(id: u32) -> sqlx::Result<EntityTable> {
    let entity_option: EntityTable = sqlx::query_as("select * from entity where id = $1")
        .bind(id)
        .fetch_one(db::get_db().await)
        .await?;

    Ok(entity_option)
}

pub async fn get_entities() -> sqlx::Result<Vec<EntityTable>> {
    let entities: Vec<EntityTable> = sqlx::query_as("select * from entity")
        .fetch_all(db::get_db().await)
        .await?;

    Ok(entities)
}

pub async fn update_entity(
    id: u32,
    tag_id: String,
    name: String,
    user_id: u32,
) -> sqlx::Result<EntityTable> {
    let entity: EntityTable = sqlx::query_as(
        "update entity set name = $1, tag_id = $2, user_id = $3 where id = $4 returning *",
    )
    .bind(name)
    .bind(tag_id)
    .bind(user_id)
    .bind(id)
    .fetch_one(db::get_db().await)
    .await?;

    Ok(entity)
}

pub async fn delete_entity(id: u32) -> sqlx::Result<()> {
    sqlx::query("delete from entity where id = $1")
        .bind(id)
        .execute(db::get_db().await)
        .await?;

    Ok(())
}

pub(crate) async fn get_entity_by_tag_id(tag_id: String) -> sqlx::Result<Option<EntityTable>> {
    let entity_option: Option<EntityTable> =
        sqlx::query_as("select * from entity where tag_id = $1")
            .bind(&tag_id)
            .fetch_optional(db::get_db().await)
            .await?;

    Ok(entity_option)
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct EntityTable {
    pub id: u32,
    pub user_id: u32,
    pub tag_id: String,
    pub name: String,
}
