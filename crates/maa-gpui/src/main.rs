mod assets;
mod root;
mod states;
mod ui;

#[async_std::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting application");

    crate::ui::app::setup().await;
}
