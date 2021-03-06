use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;

use zero2prod;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
  let default_filter_level = "info".to_string();
  let subscriber_name = "test".to_string();

  if std::env::var("TEST_LOG").is_ok() {
    let subscriber = get_subscriber(subscriber_name, default_filter_level,
      std::io::stdout);
    init_subscriber(subscriber);
  } else {
    let subscriber = get_subscriber(subscriber_name, default_filter_level,
      std::io::sink);
    init_subscriber(subscriber);
  };
});

async fn spawn_app() -> TestApp {
  Lazy::force(&TRACING);
  let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let mut configuration = get_configuration().expect("failed to read configurations");
  configuration.database.database_name = Uuid::new_v4().to_string();

  let connection_pool = configure_database(&configuration.database).await;

  let server = run(listener, connection_pool.clone()).expect("failed to bind address");
  let _ = tokio::spawn(server);

  TestApp {
    address,
    db_pool: connection_pool,
  }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
  let mut connection = PgConnection::connect(
    &config.connection_string_without_db().expose_secret()
    )
    .await
    .expect("failed to connect to database");
  connection
    .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
    .await
    .expect("failed to create database");

  let connection_pool = PgPool::connect(
    &config.connection_string().expose_secret()
    )
    .await
    .expect("failed to connect to database");
  sqlx::migrate!("./util/migrations")
    .run(&connection_pool)
    .await
    .expect("failed to migrate the database");

  connection_pool
}

#[tokio::test]
async fn health_check_works() {
  let test_app = spawn_app().await;
  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health_check", &test_app.address))
    .send()
    .await
    .expect("failed to execute request");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  let test_app = spawn_app().await;
  let client = reqwest::Client::new();

  let body = "name=le%20guin&email=ursula_le_guin%40test.com";
  let response = client
    .post(&format!("{}/subscriptions", &test_app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("failed to execute request.");

  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&test_app.db_pool)
    .await
    .expect("failed to fetch saved subscription.");

  assert_eq!(saved.email, "ursula_le_guin@test.com");
  assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  let test_app = spawn_app().await;
  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=innfi", "missing the email"),
    ("email=innfi@test.com", "missing the name"),
    ("", "missing both name and email"),
  ];

  for (invalid_body, error_message) in test_cases {
    let response = client
      .post(&format!("{}/subscriptions", &test_app.address))
      .header("Content-Type", "applications/x-www-form-urlencoded")
      .body(invalid_body)
      .send()
      .await
      .expect("failed to execute request.");

    assert_eq!(
      400,
      response.status().as_u16(),
      "api did not fail with 400 when the paylaod was {}.",
      error_message,
    );
  }
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();
  let test_cases = vec![
    ("name=&email=ursula%40gmail.com", "empty name"),
    ("name=ursula&email=", "empty email"),
    ("name=ursula&email=invalid_format", "invalid email")
  ];

  for (body, description) in test_cases {
    let response = client
      .post(&format!("{}/subscriptions", &app.address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
      .expect("failed to execute request");

      assert_eq!(
        200,
        response.status().as_u16(),
        "api did not return 200. payload: {}",
        description,
      );
  }
}