use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::env;
use std::fs;
use std::io;

pub fn configure_logging() -> anyhow::Result<()> {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse::<LevelFilter>()?;

    let log_dir = env::var("LOG_FILE_DIR").unwrap_or_else(|_| "logs".to_string());

    fs::create_dir_all(&log_dir)?;

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Blue)
        .debug(Color::Green)
        .trace(Color::Magenta);

    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}] {}: {}",
                        chrono::Local::now().format("%H:%M:%S%.3f"),
                        colors.color(record.level()),
                        message
                    ))
                })
                .chain(io::stdout()),
        )
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] {} {} - {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file(format!("{}/app.log", log_dir))?),
        )
        .level(log_level)
        .apply()?;

    Ok(())
}
