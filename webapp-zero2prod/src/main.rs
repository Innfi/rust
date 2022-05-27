use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let configurations = get_configuration().expect("failed to read configurations");

  let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind port");

  run(listener)?.await
}
