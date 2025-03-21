pub trait Logger {
    /// логирует сообщение указанного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Логировать сообщения только заданного уровняl.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity,message);//Заходим в StderrLogger, которому унаследован трейт Logger
        }
    }
}

// TODO: Реализовать типаж`Logger` для `VerbosityFilter`.

fn main() {
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "Какое то");
    logger.log(2, "Сообщение");
    logger.log(1, "Магия");
    logger.log(3, "Трейтов");
}