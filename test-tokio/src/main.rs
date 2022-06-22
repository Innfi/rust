use test_tokio::amiquip_consumer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  println!("tokio::main");

  let result = amiquip_consumer::run_consumer();
  match result {
    Err(e) => { println!("error: {}", e); },
    _ => {}
  }

  Ok(())
}
