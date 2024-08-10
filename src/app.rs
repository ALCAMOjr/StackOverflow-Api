use axum::{http::StatusCode, response::IntoResponse};
use log::info;
use sqlx::PgPool;
use std::net::SocketAddr;

pub async fn start_server(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let app = crate::routes::app_routes().fallback(fallback_handler);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!(
        "Server running on http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await?;

    Ok(())
}

async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "endpoint not found")
}
