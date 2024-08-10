use axum::{http::StatusCode, response::IntoResponse};
use log::info;
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};

use crate::persistance;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

pub async fn start_server(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone())) as Arc<dyn QuestionsDao + Send + Sync>;
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool.clone())) as Arc<dyn AnswersDao + Send + Sync>;

    let app_state = AppState {
        questions_dao,
        answers_dao,
    };

    let app = crate::routes::app_routes()
    .fallback(fallback_handler)
    .with_state(app_state);


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
