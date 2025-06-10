use gpui::{
    div, App, AppContext, BorrowAppContext, Context, Entity, IntoElement, ParentElement, Render, Styled,
    Window,
};
use gpui_component::{button::Button, v_flex, white};

use crate::settings::Settings;

pub struct TestView {}

impl TestView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self {}
    }
}

impl Render for TestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            v_flex()
                .size_full()
                .bg(white())
                .flex()
                .justify_center()
                .items_center()
                .text_3xl()
                .child(format!("Hello, world"))
                .child(
                    Button::new("test-button")
                        .label("Click me")
                        .on_click(|_, _, app| {
                            println!("Button clicked");
                            app.update_global::<Settings, ()>(|settings, _| {
                                settings.adb_path = Some(String::from("adb"));
                            })
                        }),
                ),
        )
    }
}
