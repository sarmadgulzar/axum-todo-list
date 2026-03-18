use axum::{Router, routing};

use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/", routing::get(handlers::root))
        .route("/health", routing::get(handlers::health))
}
