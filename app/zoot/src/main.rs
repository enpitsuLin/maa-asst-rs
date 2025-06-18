use assets::Assets;
use global::constants::{APP_ID, APP_NAME};
use global::paths::project_dir;
use global::shared_state;
#[cfg(target_os = "macos")]
use gpui::KeyBinding;
use gpui::{
    actions, div, prelude::*, px, size, AnyView, App, AppContext, Application, Bounds, Context, IntoElement,
    Menu, MenuItem, Render, Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui::{point, SharedString, TitlebarOptions};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{ActiveTheme, IconName, Root, Sizable, Theme, ThemeMode, TitleBar};
use maa_sys::Assistant;
use reqwest_client::ReqwestClient;
use settings::AppSettings;
use std::sync::Arc;

mod assets;
mod logger;

struct HelloWorld {
    text: SharedString,
}

impl HelloWorld {
    fn toggle_appearance(&self, window: &mut Window, cx: &mut App) {
        if cx.theme().mode.is_dark() {
            Theme::change(ThemeMode::Light, Some(window), cx);
        } else {
            Theme::change(ThemeMode::Dark, Some(window), cx);
        }
    }
    fn open_settings(&self, _window: &mut Window, _cx: &mut App) {
        tracing::info!("Opening settings");
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().relative().child(
            div()
                .flex()
                .flex_col()
                .size_full()
                .child(
                    TitleBar::new()
                        // Left side
                        .child(div())
                        // Right side
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_end()
                                .gap_1p5()
                                .px_2()
                                .child(
                                    Button::new("appearance")
                                        .tooltip("Change the app's appearance")
                                        .small()
                                        .ghost()
                                        .map(|this| {
                                            if cx.theme().mode.is_dark() {
                                                this.icon(IconName::Sun)
                                            } else {
                                                this.icon(IconName::Moon)
                                            }
                                        })
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.toggle_appearance(window, cx);
                                        })),
                                )
                                .child(
                                    Button::new("preferences")
                                        .tooltip("Open Preferences")
                                        .small()
                                        .ghost()
                                        .icon(IconName::Settings)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.open_settings(window, cx);
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .size_full()
                        .items_center()
                        .justify_center()
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
                        .child(
                            Button::new("test")
                                .label("Test")
                                .tooltip("Test")
                                .on_click(cx.listener(|_this, _, _window, cx| {
                                    AppSettings::global(cx).update(cx, |this, cx| {
                                        this.settings.foo = "baz".to_string();
                                        cx.notify();
                                    });
                                    tracing::info!("Test button clicked");
                                })),
                        )
                        .child(
                            Button::new("test-2")
                                .label("Init assistant")
                                .tooltip("Init assistant")
                                .on_click(cx.listener(|_this, _, _window, _| {
                                    tracing::info!("Test button clicked");
                                    let lib_dir = project_dir().data_dir().join("libraray");
                                    let resource_dir = project_dir().data_dir();
                                    shared_state().set_assistant(
                                        Assistant::registry()
                                            .with_library(&lib_dir)
                                            .with_resource(&resource_dir)
                                            .init()
                                            .unwrap(),
                                    );
                                })),
                        )
                        .child(
                            Button::new("test-3")
                                .label("Get assistant")
                                .tooltip("Get assistant")
                                .on_click(cx.listener(|_this, _, _window, _| {
                                    tracing::info!("Test button clicked");
                                    shared_state().with_assistant(|assistant| {
                                        tracing::info!("Assistant: {:?}", assistant.version());
                                    });
                                })),
                        ),
                ),
        )
    }
}

actions!(maa, [About, Setting, Quit]);

fn main() {
    let _guard = logger::init_logger();

    let app = Application::new()
        .with_assets(Assets)
        .with_http_client(Arc::new(ReqwestClient::new()));

    app.run(|cx| {
        // Register the `quit` function
        cx.on_action(quit);
        cx.on_action(setting);
        cx.on_action(about);

        // Register the `quit` function with CMD+Q (macOS only)
        #[cfg(target_os = "macos")]
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        // Set menu items
        cx.set_menus(vec![Menu {
            name: SharedString::new_static(APP_NAME),
            items: vec![
                MenuItem::action("Settings", Setting),
                MenuItem::action("About", About),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        }]);

        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                tracing::info!("Gracefully quitting the application . . .");
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
            tracing::info!("Starting application...");

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

                let hello_world_view = cx.new(|_| HelloWorld { text: "World".into() });

                Root::new(AnyView::from(hello_world_view), window, cx)
            })
        })
        .expect("Failed to open window. Please restart the application.");
    });
}

fn setting(_: &Setting, _cx: &mut App) {
    tracing::debug!("Opening settings");
}

fn about(_: &About, _cx: &mut App) {
    tracing::debug!("Opening about");
}

fn quit(_: &Quit, cx: &mut App) {
    tracing::info!("Gracefully quitting the application . . .");
    cx.quit();
}
