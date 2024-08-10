use axum::{
    routing::{get, post, delete},
    Router,
};
use crate::{app::AppState, handlers::*};

pub fn answers_routes() -> Router<AppState> {
    Router::new()
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
}
