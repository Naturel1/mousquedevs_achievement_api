use std::fs;
use std::path::Path;
use std::sync::Once;
use chrono::Local;
use log::LevelFilter;

static INIT: Once = Once::new();

/// Initializes the application logging system.
///
/// Configures logging output to both standard output (stdout) and a local text file.
/// Format: `[YYYY-MM-DD HH:MM:SS] [LEVEL] [TARGET] Message`
///
/// Log file path defaults to `logs/app.log` and can be overridden via the `LOG_FILE` environment variable.
/// Log level defaults to `Info` and can be overridden via `LOG_LEVEL` (trace, debug, info, warn, error).
pub fn init_logger() {
    INIT.call_once(|| {
        let log_file_path = std::env::var("LOG_FILE").unwrap_or_else(|_| "logs/app.log".to_string());
        let log_level_str = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        let level_filter = match log_level_str.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        };

        // Ensure parent log directory exists
        if let Some(parent) = Path::new(&log_file_path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).ok();
            }
        }

        let dispatch = fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}] [{}] [{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .level(level_filter)
            // Silence noisy third-party libraries if necessary, keeping app at level_filter
            .level_for("rocket", LevelFilter::Warn)
            .level_for("_", LevelFilter::Warn)
            .level_for("app", level_filter)
            .chain(std::io::stdout());

        let dispatch = match fern::log_file(&log_file_path) {
            Ok(file) => dispatch.chain(file),
            Err(err) => {
                eprintln!("Warning: Failed to open log file '{}': {}", log_file_path, err);
                dispatch
            }
        };

        if let Err(err) = dispatch.apply() {
            eprintln!("Warning: Failed to apply logger: {}", err);
        } else {
            log::info!("Logging system initialized successfully. Writing logs to: {}", log_file_path);
        }
    });
}
