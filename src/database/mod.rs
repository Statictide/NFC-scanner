pub mod entity_dao;
pub mod session_dao;
pub mod user_dao;

pub type Pool = sqlx::SqlitePool;

pub async fn get_database_pool() -> sqlx::Result<Pool> {
    let url = ":memory:";
    let pool = Pool::connect(url).await?;

    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
