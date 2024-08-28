use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::sync::OnceCell;

pub type Pool = sqlx::SqlitePool;
static POOL: OnceCell<Pool> = OnceCell::const_new();

pub async fn pool() -> &'static Pool {
    POOL.get().expect("Database pool is not initialized")
}

pub async fn init_database_pool() {
    let database_url = std::env::var("DATABASE_URL").expect("Environment variable missing: DATABASE_URL");

    let conn = SqliteConnectOptions::from_str(&database_url)
        .expect("Failed to parse DATABASE_URL")
        //.create_if_missing(true)
        ;

    let pool = SqlitePoolOptions::new()
        //.max_connections(1)
        //.idle_timeout(None)
        //.max_lifetime(None)
        .connect_with(conn)
        .await
        .expect("Failed to connect to database");

    init_pool(pool).await;
}

async fn init_pool(pool: Pool) {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    POOL.set(pool).expect("Failed to set database pool");
}
