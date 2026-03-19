use crate::tests::test_server;

#[tokio::test]
async fn test_root_endpoint() {
    let (server, _) = test_server().await;

    let response = server.get("/").await;

    response.assert_status_ok();
}
