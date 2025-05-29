use anyhow::Result;
use gpui::{
    div, px, size, white, AppContext, Application, AssetSource, Context, IntoElement, ParentElement, Render,
    SharedString, Styled, TitlebarOptions, Window, WindowDecorations, WindowOptions,
};
use gpui_component::{button::Button, hsl, Theme, ThemeColor, ThemeMode};
use std::sync::Arc;

struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(path).map(Into::into).map_err(Into::into).map(Some)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().to_string(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(white())
            .flex()
            .justify_center()
            .items_center()
            .text_3xl()
            .child(format!("Hello, {}!", &self.text))
            .child(Button::new("test-button").label("Click me").on_click(|_, _, _| {
                println!("Button clicked");
            }))
    }
}

fn main() {
    let http_client = Arc::new(reqwest_client::ReqwestClient::new());
    Application::new()
        .with_assets(Assets {})
        .with_http_client(http_client)
        .run(|app| {
            let mut theme = Theme::from(ThemeColor::dark());
            theme.mode = ThemeMode::Dark;
            theme.accent = hsl(335.0, 97.0, 61.0);
            theme.title_bar = hsl(335.0, 97.0, 61.0);
            theme.background = hsl(225.0, 12.0, 10.0);

            app.set_global(theme);

            app.open_window(
                WindowOptions {
                    window_decorations: Some(WindowDecorations::Client),
                    window_min_size: Some(size(px(800.0), px(600.0))),
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::new_static("title")),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| HelloWorld {
                        text: SharedString::new_static("World"),
                    })
                },
            )
            .unwrap();
        });
}
