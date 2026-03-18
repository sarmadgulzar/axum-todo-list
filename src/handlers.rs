use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

use crate::error::{AppError, Result};
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
) -> Result<Json<Todo>> {
    let id = Uuid::now_v7();
    let todo = sqlx::query_as!(
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
    .fetch_one(&state.db)
    .await?;

    Ok(Json(todo))
}

pub async fn get_todo(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT
            id AS "id!: _",
            title,
            completed,
            created_at AS "created_at!: _",
            updated_at AS "updated_at!: _"
        FROM todos
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::from(e),
    })?;

    Ok(Json(todo))
}
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        DELETE FROM todos
        WHERE id = $1
        RETURNING
            id AS "id!: _",
            title,
            completed,
            created_at AS "created_at!: _",
            updated_at AS "updated_at!: _"
        "#,
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::from(e),
    })?;

    Ok(Json(todo))
}

pub async fn todo_mark_complete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET completed = true, updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING
            id AS "id!: _",
            title,
            completed,
            created_at AS "created_at!: _",
            updated_at AS "updated_at!: _"
        "#,
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::from(e),
    })?;

    Ok(Json(todo))
}
pub async fn todo_mark_incomplete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET completed = false, updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING
            id AS "id!: _",
            title,
            completed,
            created_at AS "created_at!: _",
            updated_at AS "updated_at!: _"
        "#,
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::from(e),
    })?;

    Ok(Json(todo))
}
