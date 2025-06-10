use std::{fs, sync::Arc};

use directories::ProjectDirs;
use gpui::{
    actions, px, size, AnyView, App, AppContext, Application, Bounds, KeyBinding, SharedString, Window, WindowBounds,
    WindowKind, WindowOptions,
};
use gpui_component::{Root, TitleBar};
use tracing::{debug, info};

use crate::{assets::Assets, root::MAARoot, settings::Settings};

actions!(maa_actions, [Quit, Hide]);

pub async fn setup() {
    let dirs = MAAWindow::get_project_dirs();

    let directory = dirs.data_dir().to_path_buf();

    if !directory.exists() {
        fs::create_dir_all(&directory)
            .unwrap_or_else(|e| panic!("couldn't create data directory, {:?}, {:?}", directory, e));
    } else {
        info!("Data directory located at {:?}", directory);
    }

    let http_client = Arc::new(reqwest_client::ReqwestClient::user_agent("maa-gpui").unwrap());
    let app = Application::new()
        .with_assets(Assets {})
        .with_http_client(http_client);

    app.run(move |app| {
        gpui_component::init(app);

        Settings::init(app, directory.join("settings.json"));

        MAAWindow::shortcut_binding_init(app);

        app.activate(true);
        let options = MAAWindow::window_options_init(app);

        MAAWindow::windows_async_init("MAA", options, app, super::views::test::TestView::view);
    });
}

struct MAAWindow();

impl MAAWindow {
    fn windows_async_init<F, E>(title: &str, options: WindowOptions, cx: &mut App, crate_view_fn: F)
    where
        E: Into<AnyView>,
        F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
    {
        let title = SharedString::from(title.to_string());

        cx.spawn(async move |cx| {
            let window = cx
                .open_window(options, |window, cx| {
                    let view = crate_view_fn(window, cx);
                    let root = cx.new(|cx| MAARoot::new(title.clone(), view.into(), window, cx));

                    cx.new(|cx| Root::new(root.into(), window, cx))
                })
                .expect("failed to open window");

            window
                .update(cx, |_, window, _| {
                    window.activate_window();
                    window.set_window_title(&title);
                })
                .expect("failed to update window");

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    }

    fn shortcut_binding_init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", Hide, None),
        ]);
        cx.on_action(|_: &Quit, cx: &mut App| {
            cx.quit();
        });
        cx.on_action(|_: &Hide, cx: &mut App| {
            cx.hide();
        });
        debug!("shortcut binding initialized");
    }

    fn window_options_init(cx: &mut App) -> WindowOptions {
        let mut window_size = size(px(1280.0), px(960.0));
        if let Some(display) = cx.primary_display() {
            let display_size = display.bounds().size;
            window_size.width = window_size.width.min(display_size.width * 0.85);
            window_size.height = window_size.height.min(display_size.height * 0.85);
        }
        let window_bounds = Bounds::centered(None, window_size, cx);

        debug!("window options initialized");

        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(gpui::Size {
                width: px(640.),
                height: px(480.),
            }),
            kind: WindowKind::Normal,
            #[cfg(target_os = "linux")]
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            #[cfg(target_os = "linux")]
            window_decorations: Some(gpui::WindowDecorations::Client),
            ..Default::default()
        }
    }

    fn get_project_dirs() -> ProjectDirs {
        ProjectDirs::from("me", "enpitsulin", "maa-gpui").expect("couldn't find project dirs")
    }
}
