use crate::{models::*, app::AppState};
use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};

mod handlers_inner;

pub async fn create_question(
    State(state): State<AppState>, 
    Json(question): Json<Question>
) -> impl IntoResponse {
    let result = handlers_inner::create_question(question, &*state.questions_dao).await;

    match result {
        Ok(created_question) => (StatusCode::CREATED, Json(created_question)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


pub async fn read_questions(
    State(state): State<AppState>
) -> impl IntoResponse {
    let result = handlers_inner::read_questions(&*state.questions_dao).await;

    match result {
        Ok(questions) => (StatusCode::OK, Json(questions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_question(
    State(state): State<AppState>, 
    Json(question_id): Json<QuestionId>
) -> impl IntoResponse {
    let result = handlers_inner::delete_question(question_id, &*state.questions_dao).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


// ---- CRUD for Answers ----

pub async fn create_answer(
    State(state): State<AppState>, 
    Json(answer): Json<Answer>)
     -> impl IntoResponse {
    let result = handlers_inner::create_answer(answer, &*state.answers_dao).await;

    match result {
        Ok(created_answer) => (StatusCode::CREATED, Json(created_answer)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn read_answers(
    State(state): State<AppState>,
    Json(question_id): Json<QuestionId>,
) -> impl IntoResponse {

    let result = handlers_inner::read_answers(question_id, &*state.answers_dao).await;

    match result {
        Ok(anwers) => (StatusCode::OK, Json(anwers)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_answer(
    State(state): State<AppState>,
    Json(answer_id): Json<AnswerId>) 
    -> impl IntoResponse {
        let result = handlers_inner::delete_answer(answer_id, &*state.answers_dao).await;

        match result {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
}
