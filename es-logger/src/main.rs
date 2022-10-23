use es_logger::logger::{LogPayload, Logger};
use chrono::prelude::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let logger_instance = Logger::new(
    "es_test_index",
    "http://localhost:9200"
  );

  let utc: DateTime<Utc> = Utc::now();

  let _ = logger_instance.info(LogPayload {
    time: utc.timestamp(),
    log_str: "hello, world!".into(),
  }).await;

  Ok(())
}
