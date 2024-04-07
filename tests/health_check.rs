async fn spawn_app() -> Result<(), std::io::Error> {
    zero2prod::run().await
}

#[tokio::test]
async fn health_check_test() {
    // Given
    spawn_app().await.expect("Failed to spawn the app.");
    let client = reqwest::Client::new();

    // When
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
