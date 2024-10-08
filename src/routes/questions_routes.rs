use axum::{
    routing::{get, post, delete},
    Router,
};
use crate::{app::AppState, handlers::*};

pub fn questions_routes() -> Router<AppState> {
    Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question/:question_id", delete(delete_question)) 
}
