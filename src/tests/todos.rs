use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tests::{insert_todo, test_server};

#[derive(Debug, Deserialize)]
struct TodoResponse {
    id: Uuid,
    title: String,
    completed: bool,
}

#[derive(Serialize)]
struct CreateTodoBody<'a> {
    title: &'a str,
}

#[tokio::test]
async fn test_create_todo() {
    let (server, _db) = test_server().await;

    let response = server
        .post("/todos")
        .json(&CreateTodoBody { title: "buy milk" })
        .await;

    response.assert_status_ok();

    let todo: TodoResponse = response.json();
    assert_eq!(todo.title, "buy milk");
    assert!(!todo.completed);
}

#[tokio::test]
async fn test_list_todos_filters_by_title_and_completed() {
    let (server, db) = test_server().await;

    insert_todo(&db, "buy milk", false).await;
    insert_todo(&db, "buy bread", true).await;
    insert_todo(&db, "call mom", false).await;

    let response = server
        .get("/todos?title=buy&completed=true&limit=10&offset=0")
        .await;

    response.assert_status_ok();

    let todos: Vec<TodoResponse> = response.json();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "buy bread");
    assert!(todos[0].completed);
}

#[tokio::test]
async fn test_get_todo_returns_404_when_missing() {
    let (server, _db) = test_server().await;
    let id = Uuid::now_v7();

    let response = server.get(&format!("/todos/{id}")).await;

    response.assert_status(StatusCode::NOT_FOUND);
    response.assert_text("Resource not found");
}

#[tokio::test]
async fn test_mark_complete_and_incomplete() {
    let (server, db) = test_server().await;
    let id = insert_todo(&db, "buy milk", false).await;

    let complete = server.post(&format!("/todos/{id}/mark-complete")).await;

    complete.assert_status_ok();
    let todo: TodoResponse = complete.json();
    assert!(todo.completed);

    let incomplete = server.post(&format!("/todos/{id}/mark-incomplete")).await;

    incomplete.assert_status_ok();
    let todo: TodoResponse = incomplete.json();
    assert!(!todo.completed);
}

#[tokio::test]
async fn test_delete_todo_removes_row() {
    let (server, db) = test_server().await;
    let id = insert_todo(&db, "buy milk", false).await;

    let delete_response = server.delete(&format!("/todos/{id}")).await;
    delete_response.assert_status_ok();

    let deleted: TodoResponse = delete_response.json();
    assert_eq!(deleted.id, id);

    let get_response = server.get(&format!("/todos/{id}")).await;
    get_response.assert_status(StatusCode::NOT_FOUND);
}
