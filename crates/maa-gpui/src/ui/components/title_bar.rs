use gpui::{
    div, prelude::FluentBuilder, ClickEvent, Context, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, Styled, Window,
};
use gpui_component::{
    badge::Badge,
    button::{Button, ButtonVariants},
    ActiveTheme, ContextModal, IconName, Sizable, Theme, ThemeMode, TitleBar,
};

use crate::{states::app::AppStateTrait, ui::components::icon::MAAIcons};

pub struct AppTitleBar {}

impl AppTitleBar {
    pub fn new() -> Self {
        Self {}
    }

    fn change_color_mode(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let mode = match cx.theme().mode.is_dark() {
            true => ThemeMode::Light,
            false => ThemeMode::Dark,
        };

        Theme::change(mode, None, cx);
        window.refresh();
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notifications_count = window.notifications(cx).len();

        let title = cx.app_title();

        TitleBar::new()
            // left side
            .child(div().flex().items_center().child(title.clone()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(
                        Button::new("theme-mode")
                            .map(|this| {
                                if cx.theme().mode.is_dark() {
                                    this.icon(IconName::Sun)
                                } else {
                                    this.icon(IconName::Moon)
                                }
                            })
                            .small()
                            .ghost()
                            .on_click(cx.listener(Self::change_color_mode)),
                    )
                    .child(
                        Button::new("github")
                            .icon(IconName::GitHub)
                            .small()
                            .ghost()
                            .on_click(|_, _, cx| cx.open_url("https://github.com/longbridge/gpui-component")),
                    )
                    .child(
                        div().relative().child(
                            Badge::new().count(notifications_count).max(99).child(
                                Button::new("bell")
                                    .small()
                                    .ghost()
                                    .compact()
                                    .icon(MAAIcons::Bell),
                            ),
                        ),
                    )
                    .child(
                        Button::new("test")
                            .label("test")
                            .small()
                            .ghost()
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.update_title("test title");
                            })),
                    ),
            )
    }
}
