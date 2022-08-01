use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Logger {}

static INSTANCE: OnceCell<Logger> = OnceCell::new();

impl Logger {
  pub fn global() -> &'static Logger {
    println!("Logger::global() called");

    INSTANCE.get().expect("logger is not initialized")
  }

  pub fn create() -> Logger {
    Logger {}
  }

  pub fn test_info(&self, input: &str) {
    println!("{}", input);
  }
}