use test_elasticsearch::es_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let _ = es_client::es_client_single();

  Ok(())
}