#[cfg(test)]
mod tests {
    use axum_test::TestServer;

    use crate::router::create_router;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_router();
        let server = TestServer::new(app);

        let response = server.get("/health").await;

        response.assert_status_ok();
        response.assert_text("Ok");
    }
}
