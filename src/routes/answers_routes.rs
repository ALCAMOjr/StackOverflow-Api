use axum::{
    routing::{get, post, delete},
    Router,
};
use crate::handlers::*;

pub fn answers_routes() -> Router {
    Router::new()
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
}
