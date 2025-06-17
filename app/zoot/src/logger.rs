use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use global::paths::project_dir;

pub fn init_logger() -> WorkerGuard {
    let targets_filter = Targets::new().with_targets(vec![("ZOOT", Level::DEBUG)]);
    let global_env_filter = EnvFilter::try_from_env("ZOOT_LOG").unwrap_or_else(|_| {
        #[cfg(debug_assertions)]
        {
            EnvFilter::new("debug")
        }
        #[cfg(not(debug_assertions))]
        {
            EnvFilter::new("warn")
        }
    });

    let file_appender = rolling::hourly(project_dir().data_dir().join("logs"), "log");
    let (non_blocking_appender, guard) = non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender.and(std::io::stdout));

    // let fmt_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);

    tracing_subscriber::registry()
        .with(targets_filter)
        .with(global_env_filter)
        // .with(fmt_layer)
        .with(file_layer)
        .init();

    tracing::info!("Logger initialized");

    guard
}
