use tracing::{info, Level};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;
mod assets;
mod root;
mod states;
mod ui;

#[tokio::main]
async fn main() {
    let targets_filter = Targets::new().with_targets(vec![("maa_gpui", Level::DEBUG)]);

    let global_env_filter = EnvFilter::try_from_env("MAA_LOG").unwrap_or_else(|_| {
        #[cfg(debug_assertions)]
        {
            EnvFilter::new("debug")
        }
        #[cfg(not(debug_assertions))]
        {
            EnvFilter::new("warn")
        }
    });

    let fmt_layer = fmt::layer();

    tracing_subscriber::registry()
        .with(targets_filter)
        .with(fmt_layer)
        .with(global_env_filter)
        .init();

    info!("Logger initialized");

    crate::app::setup().await;
}
