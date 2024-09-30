use crate::database::{db, errors::DatabaseResult};

pub async fn get_parent_history(user_id: u32, limit: i32) -> DatabaseResult<Vec<ParentHistoryTable>> {
    let logs: Vec<ParentHistoryTable> =
        sqlx::query_as("select * from parent_history where user_id = $1 order by created_at desc limit $2 ")
            .bind(user_id)
            .bind(limit)
            .fetch_all(db::pool().await)
            .await?;

    Ok(logs)
}

pub async fn create_parent_history_entry(
    user_id: u32,
    entity_name: String,
    old_parent_name: Option<String>,
    new_parent_name: Option<String>,
) -> DatabaseResult<()> {
    sqlx::query(
        "insert into parent_history (user_id, entity_name, old_parent_name, new_parent_name) values ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(entity_name)
    .bind(old_parent_name)
    .bind(new_parent_name)
    .execute(db::pool().await)
    .await?;

    Ok(())
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct ParentHistoryTable {
    pub parent_history_id: u32,
    pub entity_name: String,
    pub old_parent_name: Option<String>,
    pub new_parent_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
