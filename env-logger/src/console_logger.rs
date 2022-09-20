use log::{Record, Level, Metadata};

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    println!("level: {}, log: {}", record.level(), record.args());
  }

  fn flush(&self) {}
}