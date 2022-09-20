use test_env_logger::console_logger::ConsoleLogger;
use log::{LevelFilter, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

fn main() -> Result<(), SetLoggerError> {
  log::set_logger(&CONSOLE_LOGGER)?;
  log::set_max_level(LevelFilter::Warn);

  log::info!("hello");
  log::warn!("warning");
  log::error!("error");

  Ok(())
}