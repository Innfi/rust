use async_once_cell::OnceCell;

#[derive(Debug)]
pub struct Logger {}

static INSTANCE: OnceCell<Logger> = OnceCell::new_with(Some(Logger {}));

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
