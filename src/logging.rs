use std::cell::{OnceCell, RefCell};

use logforth::{
    Append, append,
    record::{Level, LevelFilter},
};
use strum::EnumIs;

thread_local! {
    static LOGS: RefCell<Vec<String>> = const { RefCell::new(vec![]) };
    static MODE: OnceCell<LogMode> = const { OnceCell::new() };
}

// NOTE: Debug only, huge performance regression
#[derive(Debug)]
struct Memory;

impl Append for Memory {
    fn append(
        &self,
        record: &logforth::record::Record,
        _diags: &[Box<dyn logforth::Diagnostic>],
    ) -> Result<(), logforth::Error> {
        let level = record.level();
        let file = record.file().unwrap_or_default();
        let line = record.line().unwrap_or_default();
        let col = record.column().unwrap_or_default();
        let msg = record.payload();

        LOGS.with_borrow_mut(|v| v.push(format!("{level} {file}:{line}:{col} {msg}")));
        Ok(())
    }

    fn flush(&self) -> Result<(), logforth::Error> {
        Ok(())
    }
}

pub fn drain_logs() -> String {
    LOGS.take().join("\n")
}

#[derive(Debug, EnumIs, Clone, Copy)]
pub enum LogMode {
    Normal,
    Debug,
    Sample,
}

pub fn init_logger(mode: LogMode) {
    LOGS.replace(Vec::with_capacity(1024));
    MODE.with(|c| c.set(mode).unwrap());

    let mut log_builder = logforth::starter_log::builder().dispatch(|d| {
        d.filter(LevelFilter::MoreSevereEqual(if mode.is_sample() {
            Level::Debug
        } else {
            Level::Info
        }))
        .append(append::Stdout::default())
    });

    if !mode.is_normal() {
        log_builder =
            log_builder.dispatch(|d| d.filter(LevelFilter::Equal(Level::Debug)).append(Memory));
    };

    log_builder.apply();
}

pub fn current_mode() -> Option<LogMode> {
    MODE.with(|c| c.get().copied())
}
