use log::{Record, Level, Metadata};
use log::{SetLoggerError, LevelFilter};
use ansi_rgb::*;

struct SimpleLogger;


macro_rules! log_println {
    ($record: tt, $fg: tt) => {
        println!("{}", format_args!("{} - {}", $record.level(), $record.args()).fg($fg()))
    };
    ($record: tt, $fg: tt, $bg: tt) => {
        println!("{}", format_args!("{} - {}", $record.level(), $record.args()).fg($fg()).bg($bg()))
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Info => log_println!(record, white),
                Level::Warn => log_println!(record, yellow),
                Level::Error => log_println!(record, red),
                Level::Debug => log_println!(record, green),
                Level::Trace => log_println!(record, blue),
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

#[no_mangle]
pub unsafe extern "C" fn init_logger() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace)).unwrap();
}
