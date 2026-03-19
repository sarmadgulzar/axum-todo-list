use axum::Router;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::routing;
use std::time::Duration;
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::handlers;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<Body>, _span: &tracing::Span| {
            info!("{} {}", request.method(), request.uri());
        })
        .on_response(
            |response: &Response<Body>, latency: Duration, _span: &tracing::Span| {
                info!("{} in {:?}", response.status(), latency);
            },
        );

    Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/health", routing::get(handlers::health))
        .route(
            "/todos",
            routing::get(handlers::list_todos).post(handlers::create_todo),
        )
        .route(
            "/todos/{id}",
            routing::get(handlers::get_todo).delete(handlers::delete_todo),
        )
        .route(
            "/todos/{id}/mark-complete",
            routing::post(handlers::todo_mark_complete),
        )
        .route(
            "/todos/{id}/mark-incomplete",
            routing::post(handlers::todo_mark_incomplete),
        )
        .with_state(state)
        .layer(trace_layer)
}
