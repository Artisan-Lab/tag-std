use tracing_subscriber::{EnvFilter, fmt, prelude::*, registry};

/// Use this instead of default `RUST_LOG` to control log level.
const SAFETY_TOOL_LOG: &str = "SAFETY_TOOL_LOG";
/// If this is set to a path, logs will be written in the file.
const SAFETY_TOOL_LOG_FILE: &str = "SAFETY_TOOL_LOG_FILE";

pub fn init() {
    let fmt_layer = fmt::layer();
    let env_layer = EnvFilter::from_env(SAFETY_TOOL_LOG);
    let error_layer = tracing_error::ErrorLayer::default();

    let reg = registry().with(env_layer).with(error_layer);
    let res = if let Ok(path) = std::env::var(SAFETY_TOOL_LOG_FILE) {
        let file = std::fs::OpenOptions::new().append(true).open(path).unwrap();
        reg.with(fmt_layer.with_writer(file)).try_init()
    } else {
        reg.with(fmt_layer).try_init()
    };

    if let Err(err) = res {
        eprintln!("Logger already init: {err}");
    };

    color_eyre::install().unwrap();
}
