pub mod entity_dao;
pub mod user_dao;

pub async fn get_database_pool() -> sqlx::Result<sqlx::SqlitePool> {
    let url = ":memory:";
    let pool = sqlx::SqlitePool::connect(url).await?;

    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
