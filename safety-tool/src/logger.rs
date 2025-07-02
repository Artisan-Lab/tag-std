use tracing_subscriber::{EnvFilter, fmt, prelude::*, registry};

pub fn init() {
    let fmt_layer = fmt::layer();
    let env_layer = EnvFilter::from_default_env();
    let error_layer = tracing_error::ErrorLayer::default();

    let reg = registry().with(env_layer).with(error_layer);
    let res = if let Ok(path) = std::env::var("RUST_LOG_FILE") {
        let file = std::fs::File::create(path).unwrap();
        reg.with(fmt_layer.with_writer(file)).try_init()
    } else {
        reg.with(fmt_layer).try_init()
    };

    if let Err(err) = res {
        eprintln!("Logger already init: {err}");
    };

    color_eyre::install().unwrap();
}
