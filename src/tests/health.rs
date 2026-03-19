use crate::tests::test_server;

#[tokio::test]
async fn test_health_endpoint() {
    let (server, _) = test_server().await;

    let response = server.get("/health").await;

    response.assert_status_ok();
    response.assert_text("Ok");
}
