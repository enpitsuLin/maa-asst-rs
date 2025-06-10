use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod assets;
mod root;
mod states;
mod ui;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("MAA_LOG"))
        .init();

    info!("Logger initialized");

    crate::ui::app::setup().await;
}
