mod settings;
mod ui;
mod root;
mod title_bar;
mod assets;

#[async_std::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting application");

    crate::ui::app::setup().await;
}
