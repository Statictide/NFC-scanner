pub mod controllers;
pub mod database;
pub mod services;

use axum::routing::get;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(hello_world))
        .nest("/api", controllers::api::get_v1_api().await)
        .nest("/api", controllers::api::get_v1_api().await);

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> String {
    "Hello, World!".to_string()
}
