#[tokio::test]
async fn health_check_works() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        let app = coduck_backend::build_router();
        axum::serve(listener, app)
            .await
            .expect("Server failed to start");
    });

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://127.0.0.1:{port}/health"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let body = response.text().await.unwrap();
    assert_eq!(body, "OK");
}
