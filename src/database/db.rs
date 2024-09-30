use std::{env, str::FromStr};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::sync::OnceCell;

use crate::{database::dao::audit_log_dao::create_parent_history_entry, services::user_service};

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
    let database_url = std::env::var("DATABASE_URL").expect("Environment variable not found: DATABASE_URL");

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

    let _user = user_service::create_user("Mark".to_string(), "Static".to_string())
        .await
        .unwrap();

    let parent_id = create_entity(CreateEntity {
        tag_uid: None,
        name: "Original Grandparent".to_string(),
        parent_id: None,
    })
    .await
    .unwrap();

    let entity_id = create_entity(CreateEntity {
        tag_uid: Some("049F3972FE4A80".to_string()),
        name: "Main entity 1".to_string(),
        parent_id: Some(parent_id),
    })
    .await
    .unwrap();

    let _child1 = create_entity(CreateEntity {
        tag_uid: None,
        name: "Child 1".to_string(),
        parent_id: Some(entity_id),
    })
    .await
    .unwrap();

    let _child2 = create_entity(CreateEntity {
        tag_uid: None,
        name: "Child 2".to_string(),
        parent_id: Some(entity_id),
    })
    .await
    .unwrap();

    let _ = create_entity(CreateEntity {
        tag_uid: None,
        name: "Main entity 2".to_string(),
        parent_id: None,
    })
    .await
    .unwrap();

    let _ = create_parent_history_entry(
        1,
        "Glock".to_string(),
        Some("BankBox".to_string()),
        Some("Lasse".to_string()),
    )
    .await
    .unwrap();
}
