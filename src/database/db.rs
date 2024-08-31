use std::{env, str::FromStr};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::sync::OnceCell;

use crate::services::user_service;

pub type Pool = sqlx::SqlitePool;
static POOL: OnceCell<Pool> = OnceCell::const_new();

/**
 * Get the database pool
 */
pub async fn pool() -> &'static Pool {
    POOL.get().expect("Database pool is not initialized")
}

/**
 * Initialize the database pool
 */
pub async fn init_database_pool() {
    let database_url = std::env::var("DATABASE_URL").expect("Environment variable missing: DATABASE_URL");
    tracing::info!("Database URL: {}", database_url);

    let conn = SqliteConnectOptions::from_str(&database_url)
        .expect("Failed to parse DATABASE_URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        //.max_connections(1)
        //.idle_timeout(None)
        //.max_lifetime(None)
        .connect_with(conn)
        .await
        .expect("Failed to connect to database");

    init_pool(pool).await;

    // Add test data
    let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string());
    if (env == "dev" || env == "test") && database_url.contains(":memory:") {
        tracing::info!("Adding test data");
        add_test_data().await;
    }
}

async fn init_pool(pool: Pool) {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    POOL.set(pool).expect("Failed to set database pool");
    tracing::info!("Database pool initialized")
}

async fn add_test_data() {
    use crate::services::entity_service::create_entity;
    use crate::services::entity_service::CreateEntity;

    let user = user_service::create_user("Mark".to_string(), "Static".to_string())
        .await
        .unwrap();

    let parent_id = create_entity(CreateEntity {
        tag_uid: "0".to_string(),
        name: "Original Grandparent".to_string(),
        parent_id: None,
    })
    .await
    .unwrap();

    let entity_id = create_entity(CreateEntity {
        tag_uid: "049F3972FE4A80".to_string(),
        name: "Main entity 1".to_string(),
        parent_id: Some(parent_id),
    })
    .await
    .unwrap();

    let _child1 = create_entity(CreateEntity {
        tag_uid: "1".to_string(),
        name: "Child 1".to_string(),
        parent_id: Some(entity_id),
    })
    .await
    .unwrap();

    let _child2 = create_entity(CreateEntity {
        tag_uid: "2".to_string(),
        name: "Child 2".to_string(),
        parent_id: Some(entity_id),
    })
    .await
    .unwrap();

    let _ = create_entity(CreateEntity {
        tag_uid: "043A9F52A84A81".to_string(),
        name: "Main entity 2".to_string(),
        parent_id: None,
    })
    .await
    .unwrap();
}
