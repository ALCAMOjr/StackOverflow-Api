use axum::Router;

use crate::app::AppState;

mod questions_routes;
mod answers_routes;

pub fn app_routes() -> Router<AppState> {  // <-- Nota el uso de `AppState` aquí.
    Router::new()
        .nest("/api/", questions_routes::questions_routes())
        .nest("/api/", answers_routes::answers_routes())
}
