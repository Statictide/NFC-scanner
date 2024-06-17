pub mod controllers;
mod database;
mod services;

use axum::{http::StatusCode, routing::get};
use tower_http::trace::TraceLayer;
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    // start tracing subscriber
    tracing_subscriber::fmt::init();

    let app = axum::Router::new()
        .route("/", get("NFC Scanner"))
        .route(
            "/api",
            get("NFC Scanner API. Go to /api/v1 for the newest API."),
        )
        .nest("/api/v1", controllers::api::get_v1_api().await)
        .fallback((StatusCode::NOT_FOUND, "Not Found"))
        .layer(TraceLayer::new_for_http());

    // Cannot make IPv6 work because it infefers with android dual stack :(
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
