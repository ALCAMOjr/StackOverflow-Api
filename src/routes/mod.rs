use axum::Router;

mod questions_routes;
mod answers_routes;

pub fn app_routes() -> Router {
    Router::new()
        .nest("/api/", questions_routes::questions_routes())
        .nest("/api/", answers_routes::answers_routes())
}
