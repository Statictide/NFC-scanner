pub mod controllers;
mod database;
mod services;

use axum::{http::StatusCode, routing::get};
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(hello_world))
        .nest("/api/v1", controllers::api::get_v1_api().await)
        .fallback((StatusCode::NOT_FOUND, "Not Found"));

    // Cannot make IPv6 work because it infefers with android dual stack :(
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> String {
    "Hello, World!".to_string()
}
