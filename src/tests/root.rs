#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use sqlx::SqlitePool;

    use crate::router::create_router;
    use crate::state::AppState;

    #[tokio::test]
    async fn test_root_endpoint() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to create pool");

        let state = AppState { db: pool };

        let app = create_router(state);
        let server = TestServer::new(app);

        let response = server.get("/").await;

        response.assert_status_ok();
        response.assert_text("Hello, world");
    }
}
