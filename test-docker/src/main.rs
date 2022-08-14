use actix_web::{web, App, HttpServer, HttpResponse};
use std::net::TcpListener;
use serde::Deserialize;

#[derive(Deserialize)]
struct Confs {
  pub basic: BasicConfs,
}

#[derive(serde::Deserialize)]
struct BasicConfs {
  port: u8,
  key: String,
}

fn load_confs() -> Result<Confs, config::ConfigError> {
  let mut confs = config::Config::default();
  let base_path = std::env::current_dir().expect("path error");
  let conf_dir = base_path.join("conf");

  confs.merge(config::File::from(conf_dir.join("base")).required(true))?;

  confs.try_into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let confs = load_confs().expect("failed to find conf files");
  println!("load confs for {}", confs.basic.key);

  let url = format!("127.0.0.1:{}", confs.basic.port);
  let listener = TcpListener::bind(url.as_str())?;
  println!("starting http server");

  let _ = HttpServer::new(move || {
    App::new().route("/", web::get().to(initial_handler)) 
  })
  .listen(listener)?
  .run().await;

  Ok(())
}

async fn initial_handler() -> HttpResponse {
  HttpResponse::Ok().finish()
}
