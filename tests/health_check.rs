use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // get the local port we bound to
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to  bind address.");
    let _ = tokio::spawn(server);
    // return the address to caller
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_test() {
    // Given
    let address = spawn_app();
    let client = reqwest::Client::new();

    // When
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
