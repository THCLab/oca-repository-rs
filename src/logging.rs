use std::path::Path;
use std::sync::Once;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub enum LogOutput {
    Stderr,
    File(Option<String>),
}

static LOG_INIT: Once = Once::new();

pub fn init_tracing(output: LogOutput) {
    LOG_INIT.call_once(|| {
        let _ = LogTracer::init();

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        match output {
            LogOutput::Stderr => {
                let fmt_layer = fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_target(true)
                    .with_level(true);
                let _ = tracing_subscriber::registry()
                    .with(filter)
                    .with(fmt_layer)
                    .try_init();
            }
            LogOutput::File(path) => {
                let default_path = "/var/log/oca-repository.log".to_string();
                let path_to_use = path.unwrap_or(default_path);
                if let Some(dir) = Path::new(&path_to_use).parent() {
                    std::fs::create_dir_all(dir).ok();
                }
                // Daily rotation in that file’s directory
                let file_appender = RollingFileAppender::new(
                    Rotation::DAILY,
                    Path::new(&path_to_use).parent().unwrap(),
                    Path::new(&path_to_use).file_name().unwrap(),
                );
                let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

                let fmt_layer = fmt::layer()
                    .with_writer(non_blocking)
                    .with_target(true)
                    .with_level(true)
                    .with_ansi(false); // disable colors in files

                let _ = tracing_subscriber::registry()
                    .with(filter)
                    .with(fmt_layer)
                    .try_init();

                // keep _guard in scope if you want guaranteed flushing at shutdown
                std::mem::forget(_guard);
            }
        }
    })
}
