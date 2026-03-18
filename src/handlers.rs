use axum::extract::State;
use axum::response::IntoResponse;

use crate::state::AppState;

use super::error::Result;

pub async fn root(State(_state): State<AppState>) -> Result<&'static str> {
    Ok("Hello, world")
}

pub async fn health(State(_state): State<AppState>) -> Result<&'static str> {
    Ok("Ok")
}
