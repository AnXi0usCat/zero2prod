use reqwest::StatusCode;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};

struct TestApp {
    pub address: String,
    pub pg_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // get the local port we bound to
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to get configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let pool = configure_database(&configuration.database).await;
    let server = run(listener, pool.clone()).expect("Failed to  bind address.");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        pg_pool: pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to database");
    connection
        .execute(format!(r#"CREATE DATABASE "{}" "#, config.database_name).as_str())
        .await
        .expect("FAiled to create databse");

    // Migrate databse
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Migrate datatbase
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[tokio::test]
async fn health_check_returns_200() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // When
    let response = client
        .get(&format!("{}/health_check", app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let data = "name=meow%20purf&email=meow_purf%40gmail.com";

    // When
    let response = client
        .post(&format!("{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    // test postgres
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pg_pool)
        .await
        .expect("Failed to fetch failed subscription.");

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(saved.email, "meow_purf@gmail.com");
    assert_eq!(saved.name, "meow purf");
}

#[tokio::test]
async fn subscribe_reutrns_400_when_data_is_missing() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=meow%20", "missing the email"),
        ("email=meow_purf%40@gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    //When
    for (test, message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", app.address))
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
