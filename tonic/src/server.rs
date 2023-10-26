use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};

pub mod hello_world {
  tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
  async fn say_hello(
    &self, 
    request: Request<HelloRequest>,
  ) -> Result<Response<HelloResponse>, Status> {
    println!("request: {:?}", request);

    Ok(Response::new(HelloResponse {
      message: format!("default response").into(),
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;
  let greeter = MyGreeter::default();

  Server::builder()
    .add_service(GreeterServer::new(greeter))
    .serve(addr)
    .await?;

  Ok(())
}