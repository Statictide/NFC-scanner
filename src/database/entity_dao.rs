pub async fn create_entity(
    tag_id: String,
    name: Option<String>,
    owner: Option<String>,
    db: &sqlx::SqlitePool,
) -> sqlx::Result<EntityTable> {
    let entity: EntityTable =
        sqlx::query_as("insert into entity (tag_id, name, owner) values ($1, $2, $3) returning *")
            .bind(tag_id)
            .bind(name)
            .bind(owner)
            .fetch_one(db)
            .await?;

    Ok(entity)
}

pub async fn get_entity(id: u32, db: &sqlx::SqlitePool) -> sqlx::Result<EntityTable> {
    let entity_option: EntityTable = sqlx::query_as("select * from entity where id = $1")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(entity_option)
}

pub async fn get_entities(db: &sqlx::SqlitePool) -> sqlx::Result<Vec<EntityTable>> {
    let entities: Vec<EntityTable> = sqlx::query_as("select * from entity").fetch_all(db).await?;

    Ok(entities)
}

pub async fn update_entity(
    id: u32,
    tag_id: String,
    name: String,
    owner: String,
    db: &sqlx::SqlitePool,
) -> sqlx::Result<EntityTable> {
    let entity: EntityTable = sqlx::query_as(
        "update entity set tag_id = $1, name = $2, owner = $3 where id = $4 returning *",
    )
    .bind(tag_id)
    .bind(name)
    .bind(owner)
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(entity)
}

pub async fn delete_entity(id: u32, db: &sqlx::SqlitePool) -> sqlx::Result<()> {
    sqlx::query("delete from entity where id = $1")
        .bind(id)
        .execute(db)
        .await?;

    Ok(())
}

pub(crate) async fn get_entity_by_tag_id(
    tag_id: String,
    db: &sqlx::SqlitePool,
) -> sqlx::Result<Option<EntityTable>> {
    let entity_option: Option<EntityTable> =
        sqlx::query_as("select * from entity where tag_id = $1")
            .bind(&tag_id)
            .fetch_optional(db)
            .await?;

    Ok(entity_option)
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct EntityTable {
    pub id: u32,
    pub tag_id: String,
    pub name: String,
    pub owner: String,
}
