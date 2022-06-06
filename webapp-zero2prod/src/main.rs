use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  // LogTracer::init().expect("failed to set logger");

  // let env_filter = EnvFilter::try_from_default_env()
  //   .unwrap_or_else(|_| EnvFilter::new("info"));
  // let formatting_layer = BunyanFormattingLayer::new(
  //   "zero2prod".into(),
  //   std::io::stdout
  // );
  // let subscriber = Registry::default()
  //   .with(env_filter)
  //   .with(JsonStorageLayer)
  //   .with(formatting_layer);
  // set_global_default(subscriber).expect("failed to set subscriber");
  let subscriber = get_subscriber("zero2prod".into(), "info".into());
  init_subscriber(subscriber);

  let configuration = get_configuration().expect("failed to read configurations");

  let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("failed to connect to database");

  let address = format!("127.0.0.1:{}", configuration.application_port);
  let listener = TcpListener::bind(address)?;

  run(listener, connection_pool)?.await
}
