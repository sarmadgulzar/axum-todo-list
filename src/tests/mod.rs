pub mod health;
pub mod root;
pub mod todos;

use axum_test::TestServer;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use uuid::Uuid;

use crate::router::create_router;
use crate::state::AppState;

pub async fn test_server() -> (TestServer, SqlitePool) {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = create_router(AppState { db: pool.clone() });
    let server = TestServer::new(app);

    (server, pool)
}

pub async fn insert_todo(pool: &SqlitePool, title: &str, completed: bool) -> Uuid {
    let id = Uuid::now_v7();

    sqlx::query("INSERT INTO todos (id, title, completed) VALUES (?, ?, ?)")
        .bind(id)
        .bind(title)
        .bind(completed)
        .execute(pool)
        .await
        .expect("Failed to insert todo");

    id
}
