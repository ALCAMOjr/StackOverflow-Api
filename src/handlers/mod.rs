use crate::models::*;
use axum::{
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde_json::json;
// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    // Implementar la lógica para crear una pregunta.
    // Puedes devolver un estado HTTP 201 (Created) junto con la pregunta creada.
    Json(question) // Esto es solo un ejemplo de respuesta
}

pub async fn read_questions() -> impl IntoResponse {
    // Crear un objeto JSON con el mensaje "Hola mundo"
    let response_body = json!({ "message": "Hola mundo" });

    // Devolver el JSON con un código de estado 200 OK
    (StatusCode::OK, Json(response_body))
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    // Implementar la lógica para eliminar una pregunta por su UUID.
    // Puedes devolver un estado HTTP 204 (No Content) para indicar que la operación fue exitosa.
    ()
}

// ---- CRUD for Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    // Implementar la lógica para crear una respuesta.
    // Devolver el detalle de la respuesta creada como JSON.
    Json(answer) // Esto es solo un ejemplo de respuesta
}

pub async fn read_answers(Json(question_id): Json<QuestionId>) -> impl IntoResponse {
    // Implementar la lógica para leer todas las respuestas asociadas a una pregunta.
    // Devolver un vector con los detalles de las respuestas como JSON.
}

pub async fn delete_answer(Json(answer_id): Json<AnswerId>) -> impl IntoResponse {
    // Implementar la lógica para eliminar una respuesta por su ID.
    // Puedes devolver un estado HTTP 204 (No Content) para indicar que la operación fue exitosa.
    ()
}
