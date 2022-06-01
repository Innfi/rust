use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use sqlx::PgPool;

use zero2prod;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
  let listener = TcpListener::bind("127.0.0.1:0")
    .expect("failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let configuration = get_configuration().expect("failed to read configurations");
  let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("failed to connect to database");

  let server = run(listener, connection_pool.clone())
    .expect("failed to bind address");
  let _ = tokio::spawn(server);

  //format!("http://127.0.0.1:{}", port)
  TestApp {
    address,
    db_pool: connection_pool,
  }
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

// #[tokio::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//   let app_address = spawn_app().await;

//   let configuration = get_configuration().expect("failed to read conf");
//   let connection_string = configuration.database.connection_string();
//   let mut connection = PgConnection::connect(&connection_string)
//     .await
//     .expect("failed to connect database");

//   let client = reqwest::Client::new();

//   let body = "name=le%20guin&email=ursula_le_guin%40test.com";
//   let response = client
//     .post(&format!("{}/subscriptions", &app_address))
//     .header("Content-Type", "application/x-www-form-urlencoded")
//     .body(body)
//     .send()
//     .await
//     .expect("failed to execute request.");

//   assert_eq!(200, response.status().as_u16());

//   let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
//     .fetch_one(&mut connection)
//     .await
//     .expect("failed to fetch saved subscription.");

//   assert_eq!(saved.email, "ursula_le_guin%40test.com");
//   assert_eq!(saved.name, "le guin");
// }

// #[tokio::test]
// async fn subscribe_returns_a_400_when_data_is_missing() {
//   let app_address = spawn_app().await;
//   let client = reqwest::Client::new();

//   let test_cases = vec![
//     ("name=innfi", "missing the email"),
//     ("email=innfi@test.com", "missing the name"),
//     ("", "missing both name and email"),
//   ];

//   for (invalid_body, error_message) in test_cases {
//     let response = client
//       .post(&format!("{}/subscriptions", &app_address))
//       .header("Content-Type", "applications/x-www-form-urlencoded")
//       .body(invalid_body)
//       .send()
//       .await
//       .expect("failed to execute request.");

//     assert_eq!(
//       400,
//       response.status().as_u16(),
//       "api did not fail with 400 when the paylaod was {}.",
//       error_message,
//     );
//   }
// }
