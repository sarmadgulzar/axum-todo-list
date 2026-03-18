use axum_todo_list::router;
use axum_todo_list::state::AppState;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let options = SqliteConnectOptions::from_str("sqlite:todo.db")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
        .await
        .expect("Failed to connect to SQLite");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState { db: pool };
    let app = router::create_router(state);

    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;
    let listner = TcpListener::bind((addr, port)).await.unwrap();

    axum::serve(listner, app).await.unwrap();
}
