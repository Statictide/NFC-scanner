use std::net::SocketAddr;

use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(hello_world))
        .nest("/api/v1", api::get_api_routes().await);

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> String {
    "Hello, World!".to_string()
}

mod api {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::routing::{get, post};
    use axum::{Json, Router};
    use sqlx::SqlitePool;

    use crate::{database, entity_service};

    pub async fn get_api_routes() -> Router {
        let pool = database::get_database_pool().await.unwrap();

        Router::new()
            .route("/", get("hello world"))
            .nest("/entity", entity_routes().await)
            .with_state(pool)
    }

    pub async fn entity_routes() -> Router<SqlitePool> {
        Router::new()
            .route("/", post(create_entity))
            .route("/:id", get(get_entity))
    }

    async fn create_entity(State(pool): State<SqlitePool>, Json(create_entity): axum::extract::Json<CreateEntity>) -> impl IntoResponse {
        let entity = entity_service::create_entity(create_entity.into(), &pool).await;

        let entity_dto = EntityDTO::from(entity);
        (StatusCode::CREATED, Json(entity_dto))
    }

    async fn get_entity(Path(id): Path<u32>, State(pool): State<SqlitePool>) -> impl IntoResponse {
        let entity = entity_service::get_entity(id, &pool).await.unwrap();
        let entity_dto = EntityDTO::from(entity);
        (StatusCode::OK, Json(entity_dto))
    }

    
    #[derive(serde::Deserialize)]
    pub struct CreateEntity {
        name: String,
        owner: String,
    }

    impl From<CreateEntity> for entity_service::CreateEntity {
        fn from(create_entity: CreateEntity) -> Self {
            Self {
                name: create_entity.name,
                owner: create_entity.owner,
            }
        }
    }

    #[derive(serde::Serialize)]
    pub struct EntityDTO {
        id: u32,
        name: String,
        owner: String,
    }

    use crate::entity_service::Entity;
    impl From<Entity> for EntityDTO {
        fn from(entity: Entity) -> Self {
            Self {
                id: entity.id,
                name: entity.name,
                owner: entity.owner,
            }
        }
    }
}

mod entity_service {
    use crate::database::entity_dao;

    pub async fn create_entity(create_entity: CreateEntity, pool: &sqlx::SqlitePool) -> Entity {
        let entity_table = entity_dao::create_entity(create_entity.name, create_entity.owner, pool)
            .await
            .unwrap();

        return Entity::from_entity_table(entity_table);
    }

    pub async fn get_entity(id: u32, pool: &sqlx::SqlitePool) -> Option<Entity> {
        let entity_table = entity_dao::get_entity(id, pool).await.unwrap();
        let entity = Entity::from_entity_table(entity_table);
        Some(entity)
    }

    #[derive(serde::Deserialize)]
    pub struct CreateEntity {
        pub name: String,
        pub owner: String,
    }

    pub struct Entity {
        pub id: u32,
        pub name: String,
        pub owner: String,
    }

    impl Entity {
        pub fn from_entity_table(entity: entity_dao::EntityTable) -> Self {
            Self {
                id: entity.id,
                name: entity.name,
                owner: entity.owner,
            }
        }
    }
}

mod database {
    pub async fn get_database_pool() -> sqlx::Result<sqlx::SqlitePool> {
        let url = ":memory:";
        let pool = sqlx::SqlitePool::connect(url).await?;

        sqlx::migrate!().run(&pool).await?;
        Ok(pool)
    }

    pub mod entity_dao {
        pub async fn create_entity(
            name: String,
            owner: String,
            pool: &sqlx::SqlitePool,
        ) -> sqlx::Result<EntityTable> {
            let entity: EntityTable =
                sqlx::query_as("insert into entity (name, owner) values ($1, $2) returning *")
                    .bind(name)
                    .bind(owner)
                    .fetch_one(pool)
                    .await?;

            Ok(entity)
        }

        pub async fn get_entity(id: u32, pool: &sqlx::SqlitePool) -> sqlx::Result<EntityTable> {
            let entity: EntityTable = sqlx::query_as("select * from entity where id = $1")
                .bind(id)
                .fetch_one(pool)
                .await?;

            Ok(entity)
        }

        #[derive(serde::Serialize, sqlx::FromRow)]
        pub struct EntityTable {
            pub id: u32,
            pub name: String,
            pub owner: String,
        }
    }
}
