use assets::Assets;
use global::constants::{APP_ID, APP_NAME};
use gpui::{
    actions, div, prelude::*, px, rgb, size, App, AppContext, Application, Bounds, Context, IntoElement,
    KeyBinding, Menu, MenuItem, Render, Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui::{point, SharedString, TitlebarOptions};
use gpui_component::Theme;
use reqwest_client::ReqwestClient;
use std::sync::Arc;

mod assets;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size_full()
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(gpui::red()))
                    .child(div().size_8().bg(gpui::green()))
                    .child(div().size_8().bg(gpui::blue()))
                    .child(div().size_8().bg(gpui::yellow()))
                    .child(div().size_8().bg(gpui::black()))
                    .child(div().size_8().bg(gpui::white())),
            )
    }
}

actions!(maa, [Quit]);

fn main() {
    tracing_subscriber::fmt::init();

    let app = Application::new()
        .with_assets(Assets)
        .with_http_client(Arc::new(ReqwestClient::new()));

    app.run(|cx| {
        // Register the `quit` function
        cx.on_action(quit);

        // Register the `quit` function with CMD+Q (macOS only)
        #[cfg(target_os = "macos")]
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        // Set menu items
        cx.set_menus(vec![Menu {
            name: "MAA".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let mut window_size = size(px(1280.0), px(960.0));
        if let Some(display) = cx.primary_display() {
            let display_size = display.bounds().size;
            window_size.width = window_size.width.min(display_size.width * 0.85);
            window_size.height = window_size.height.min(display_size.height * 0.85);
        }
        let window_bounds = Bounds::centered(None, window_size, cx);

        let opts = WindowOptions {
            #[cfg(not(target_os = "linux"))]
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::new_static(APP_NAME)),
                traffic_light_position: Some(point(px(9.0), px(9.0))),
                appears_transparent: true,
            }),
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            window_min_size: Some(size(px(860.0), px(640.0))),
            #[cfg(target_os = "linux")]
            window_background: WindowBackgroundAppearance::Transparent,
            #[cfg(target_os = "linux")]
            window_decorations: Some(WindowDecorations::Client),
            kind: WindowKind::Normal,
            app_id: Some(APP_ID.to_owned()),
            ..Default::default()
        };

        // Open a window with default options
        cx.open_window(opts, |window, cx| {
            // Automatically sync theme with system appearance
            window
                .observe_window_appearance(|window, cx| {
                    Theme::sync_system_appearance(Some(window), cx);
                })
                .detach();

            // Root Entity
            cx.new(|cx| {
                cx.activate(true);

                // Initialize settings
                settings::init(cx);
                // Initialize components
                gpui_component::init(cx);

                HelloWorld { text: "World".into() }
            })
        })
        .expect("Failed to open window. Please restart the application.");
    });
}

fn quit(_: &Quit, cx: &mut App) {
    tracing::info!("Gracefully quitting the application . . .");
    cx.quit();
}
