extern crate ci_build;

extern crate log;

fn main() {
    log::set_logger(|max_log_level| {
        max_log_level.set(::log::LogLevelFilter::Debug);
        Box::new(SimpleLogger)
    }).unwrap();
    ci_build::perform_build("tests/resources/test_config.yml")
        .unwrap_or_else(|err| println!("An error has occured! {:?}", err))
}

use log::{LogLevel, LogMetadata, LogRecord};

struct SimpleLogger;

impl ::log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}
