use tokio::sync::OnceCell;
use super::Pool;

static STATIC_POOL: OnceCell<Pool> = OnceCell::const_new();

pub async fn get_db() -> &'static Pool {
    STATIC_POOL.get_or_init(|| async {
        init_database_pool().await.expect("Failed to connect to database")
    }).await
}

pub async fn init_database_pool() -> sqlx::Result<Pool> {
    //let database_url = ":memory:";
    let database_url = "sqlite://database.db";
    let pool = Pool::connect(database_url).await?;

    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
