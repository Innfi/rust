use log4rs::init_file;

fn main() {
  init_file(
    "log4rs.yaml",
    log4rs_logstash::config::deserializers(),
  ).unwrap();

  log::debug!("debug");
  log::trace!("trace");
  log::info!("info");
  log::warn!("warn");
  log::error!("error");

  println!("Hello, world!");
}
