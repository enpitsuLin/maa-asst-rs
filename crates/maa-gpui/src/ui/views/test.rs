use gpui::{div, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{button::Button, v_flex, white, ContextModal};
use tracing::info;

use crate::states::settings::SettingsTrait;

pub struct TestView {}

impl TestView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self {}
    }

    fn show_modal(&mut self, window: &mut Window, cx: &mut App) {
        window.open_modal(cx, move |modal, _, _| {
            modal
                .title("Test Modal")
                .child(
                    v_flex()
                        .gap_3()
                        .child("This is a modal dialog.")
                        .child("You can put anything here."),
                )
                .footer(|render_ok, render_cancel, window, cx| {
                    vec![render_ok(window, cx), render_cancel(window, cx)]
                })
        });
    }
}

impl Render for TestView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            v_flex()
                .size_full()
                .bg(white())
                .flex()
                .justify_center()
                .items_center()
                .text_3xl()
                .gap_2()
                .child(serde_json::to_string(&cx.settings()).unwrap())
                .child(format!("Hello, world"))
                .child(
                    Button::new("test-button")
                        .label("Click me")
                        .on_click(|_, _, app| {
                            info!("Button clicked");
                            let settings = app.update_settings(|settings, _| {
                                settings.adb_path = Some(String::from("adb"));

                                serde_json::to_string(settings).unwrap()
                            });

                            info!("settings: {:?}", settings);
                        }),
                )
                .child(
                    Button::new("test-modal")
                        .label("Open Modal")
                        .on_click(cx.listener(|this, _, window, cx| this.show_modal(window, cx))),
                ),
        )
    }
}
