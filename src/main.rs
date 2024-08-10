use std::net::SocketAddr;
use axum::{
    http::StatusCode,
    response::IntoResponse,
};

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::app_routes().fallback(fallback_handler);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "endpoint not found")
}
