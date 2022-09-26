use es_logger::logger::Logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let logger_instance = Logger::new("http://localhost:9200");

  let _ = logger_instance.info("hello, world!").await;

  Ok(())
}
