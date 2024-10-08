use crate::{models::*, app::AppState};
use axum::{
    extract::{Json, Path, State}, http::StatusCode, response::IntoResponse
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
    Path(question_id): Path<String> 
) -> impl IntoResponse {
    let question_id = QuestionId { question_uuid: question_id };
  
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
    Path(question_id): Path<String> 
) -> impl IntoResponse {
    let question_id = QuestionId { question_uuid: question_id };
    let result = handlers_inner::read_answers(question_id, &*state.answers_dao).await;

    match result {
        Ok(answers) => (StatusCode::OK, Json(answers)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_answer(
    State(state): State<AppState>,
    Path(answer_id): Path<String> 
)
    -> impl IntoResponse {
        let answer_id = AnswerId {  answer_uuid: answer_id };
        let result: Result<(), handlers_inner::HandlerError> = handlers_inner::delete_answer(answer_id, &*state.answers_dao).await;

        match result {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
}
