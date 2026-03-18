use axum::Json;
use axum::extract::State;
use uuid::Uuid;

use crate::error::Result;
use crate::models::Todo;
use crate::schemas::CreateTodo;
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

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Vec<Todo>>> {
    let id = Uuid::now_v7();
    let todos = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (id, title)
        VALUES ($1, $2)
        RETURNING
            id AS "id!: _",
            title,
            completed,
            created_at AS "created_at!: _",
            updated_at AS "updated_at!: _"
        "#,
        id,
        payload.title
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(todos))
}
