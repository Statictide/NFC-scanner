mod controllers;
mod database;
mod services;
mod util;

use axum::{http::StatusCode, response::IntoResponse, routing::get};
use database::db;
use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    // Log info
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize database pool upfront
    db::init_database_pool().await;

    // https://docs.rs/tower-http/latest/tower_http/trace/index.html
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::DEBUG))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
        .on_failure(DefaultOnFailure::new().level(Level::ERROR));

    let app = axum::Router::new()
        .route("/", get("NFC Scanner"))
        .nest("/api/v0", controllers::api::get_v0_api().await)
        .fallback(fallback)
        .layer(trace_layer);

    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Failed to parse PORT");

    // Cannot make IPv6 work because it infers with android dual stack :(
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {addr}. See http://localhost:{}", addr.port());
    axum::serve(listener, app).await.unwrap();
}

async fn fallback(body: String) -> impl IntoResponse {
    tracing::info!("Fallback: {}", body);
    (StatusCode::NOT_FOUND, "Route not Found")
}
