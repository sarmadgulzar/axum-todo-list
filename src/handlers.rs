use axum::Json;
use axum::extract::State;

use crate::error::Result;
use crate::models::Todo;
use crate::state::AppState;

pub async fn root(State(_state): State<AppState>) -> Result<&'static str> {
    Ok("Hello, world")
}

pub async fn health(State(_state): State<AppState>) -> Result<&'static str> {
    Ok("Ok")
}

pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT
            id AS "id!: _",
            title,
            completed,
            created_at AS "created_at!: _",
            updated_at AS "updated_at!: _"
        FROM todos
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(todos))
}
