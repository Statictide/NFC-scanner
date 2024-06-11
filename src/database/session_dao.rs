use crate::database::db;

pub async fn get_session(user_id: u32) -> sqlx::Result<Option<SessionTable>> {
    let pool = db::get_db().await;
    let session_option: Option<SessionTable> =
        sqlx::query_as("select * from session where user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

    Ok(session_option)
}

pub async fn create_session(user_id: u32, token: String) -> sqlx::Result<SessionTable> {
    let pool = db::get_db().await;
    let session: SessionTable = sqlx::query_as(
        "insert into session (user_id, token) values ($1, $2) returning *",
    )
    .bind(user_id)
    .bind(token)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

#[derive(sqlx::FromRow)]
pub struct SessionTable {
    pub id: u32,
    pub user_id: u32,
    pub token: String,
}

pub async fn _get_session_by_token(token: String) -> sqlx::Result<Option<SessionTable>> {
    let pool = db::get_db().await;
    let session: Option<SessionTable> = sqlx::query_as("select * from session where token = $1")
        .bind(token)
        .fetch_optional(pool)
        .await?;

    Ok(session)
}
