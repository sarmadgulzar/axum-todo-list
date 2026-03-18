use axum::{Router, routing};

use crate::handlers;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(handlers::root))
        .route("/health", routing::get(handlers::health))
        .route(
            "/todos",
            routing::get(handlers::list_todos).post(handlers::create_todo),
        )
        .with_state(state)
}
