use sqlx::pool::PoolOptions;
use tokio::sync::OnceCell;

pub type Pool = sqlx::SqlitePool;
static POOL: OnceCell<Pool> = OnceCell::const_new();

pub async fn get_db() -> &'static Pool {
    POOL.get().expect("Database pool is not initialized")
}

#[allow(dead_code)]
pub enum DatabaseType {
    InMemory,
    InFile,
}

pub async fn init_database_pool(database_type: DatabaseType) -> Result<(), sqlx::Error> {
    match database_type {
        DatabaseType::InMemory => init_database_pool_in_memory().await,
        DatabaseType::InFile => init_database_pool_in_file("database.db").await,
    }
}

async fn init_database_pool_in_memory() -> Result<(), sqlx::Error> {
    let database_url = "sqlite://:memory:";
    let pool: Pool = PoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect(&database_url)
        .await?;

    init(pool).await
}

async fn init_database_pool_in_file(db_file_path: &str) -> Result<(), sqlx::Error> {
    let database_url = format!("sqlite://{db_file_path}");
    let pool = Pool::connect(&database_url).await?;

    init(pool).await
}

async fn init(pool: Pool) -> Result<(), sqlx::Error> {
    sqlx::migrate!().run(&pool).await?;

    POOL.set(pool).expect("Database already initialized");
    Ok(())
}
