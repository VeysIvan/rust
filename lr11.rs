pub trait Logger {
  /// Помещает в лог сообщения заданного уровня.
  fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
  fn log(&self, verbosity: u8, message: &str) {
      eprintln!("verbosity={verbosity}: {message}");
  }
}

// TODO: Добавьте определение и реализацию Filter.
struct Filter<F: Fn(u8, &str) -> bool> {
  inner: StderrLogger,
  filter: F,
}

impl<F: Fn(u8, &str) -> bool> Filter<F> {
  fn new(inner: StderrLogger, filter: F) -> Self {
      Filter { inner, filter }
  }
}

impl<F: Fn(u8, &str) -> bool> Logger for Filter<F> {
  fn log(&self, verbosity: u8, message: &str) {
      if (self.filter)(verbosity, message) {
          self.inner.log(verbosity, message);
      }
  }
}

fn main() {
  let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
  logger.log(5, "FYI");
  logger.log(1, "yikes, something went wrong");
  logger.log(2, "uhoh");
}
