use axum::{
    routing::{get, post, delete},
    Router,
};
use crate::handlers::*;

pub fn questions_routes() -> Router {
    Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
}
