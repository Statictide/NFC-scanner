pub mod controllers;
mod database;
mod services;

use axum::{http::StatusCode, routing::get};
use database::db;
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Start tracing subscriber
    tracing_subscriber::fmt::init();

    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize database pool upfront
    db::init_database_pool().await;

    let app = axum::Router::new()
        .route("/", get("NFC Scanner"))
        .nest("/api/v0", controllers::api::get_v0_api().await)
        .fallback((StatusCode::NOT_FOUND, "Route not Found"))
        .layer(TraceLayer::new_for_http());

    // Cannot make IPv6 work because it infers with android dual stack :(
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {addr}. See http://localhost:{}", addr.port());
    axum::serve(listener, app).await.unwrap();
}
