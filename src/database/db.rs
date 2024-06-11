use tokio::sync::OnceCell;
use super::Pool;

static STATIC_POOL: OnceCell<Pool> = OnceCell::const_new();

pub async fn get_db() -> &'static Pool {
    STATIC_POOL.get_or_init(|| async {
        init_database_pool().await.expect("Failed to initialize database pool")
    }).await
}

async fn init_database_pool_fallible() -> Pool {
    init_database_pool()
        .await
        .expect("Failed to initialize database pool")
}

pub async fn init_database_pool() -> sqlx::Result<Pool> {
    let url = ":memory:";
    let pool = Pool::connect(url).await?;

    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
