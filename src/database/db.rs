use sqlx::{pool::PoolOptions, SqlitePool};
use tokio::sync::OnceCell;

pub type Pool = sqlx::SqlitePool;
static POOL: OnceCell<Pool> = OnceCell::const_new();

pub async fn pool() -> &'static Pool {
    POOL.get().expect("Database pool is not initialized")
}

pub async fn init_database_pool(database_url: &str) -> Result<(), sqlx::Error> {
        let database_url = format!("{database_url}");
        let pool = SqlitePool::connect(&database_url).await?;

        init_pool(pool).await
}


async fn init_pool(pool: Pool) -> Result<(), sqlx::Error> {
    sqlx::migrate!().run(&pool).await?;

    POOL.set(pool).expect("Database already initialized");
    Ok(())
}



async fn _init_database_pool_in_memory() -> Result<(), sqlx::Error> {
    let database_url = "sqlite://:memory:";
    let pool: Pool = PoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect(&database_url)
        .await?;

    init_pool(pool).await
}
