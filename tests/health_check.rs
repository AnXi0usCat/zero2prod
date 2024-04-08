use std::net::TcpListener;

use reqwest::StatusCode;

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
async fn health_check_returns_200() {
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

#[tokio::test]
async fn subscrine_returns_200_for_valid_form_data() {
    // Given
    let address = spawn_app();
    let client = reqwest::Client::new();
    let data = "name=meow%20purf&email=meow_purf%40@gmail.com";

    // When
    let response = client
        .post(&format!("{}/subscribe", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert_eq!(StatusCode::OK, response.status())
}

#[tokio::test]
async fn subscribe_reutrns_400_when_data_is_missing() {
    // Given
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=meow%20", "missing the email"),
        ("email=meow_purf%40@gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    //When
    for (test, message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(test)
            .send()
            .await
            .expect("Failed to execute request.");

        // Then
        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "The API did not fail with status 400 when payload was {}",
            message
        )
    }
}
