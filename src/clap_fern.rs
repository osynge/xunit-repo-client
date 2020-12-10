use chrono;
use fern;
use log;
use std;

pub fn log_setup(config: &crate::config::Config) -> () {
    let default = log::LevelFilter::Info;
    let log_level = match config.loglevel {
        Some(p) => {
            if p < -2 {
                log::LevelFilter::Off
            } else if p == -2 {
                log::LevelFilter::Error
            } else if p == -1 {
                log::LevelFilter::Warn
            } else if p == 0 {
                log::LevelFilter::Info
            } else if p == 1 {
                log::LevelFilter::Debug
            } else if p == 2 {
                log::LevelFilter::Trace
            } else if p > 2 {
                log::LevelFilter::Trace
            } else {
                default
            }
        }
        None => default,
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
