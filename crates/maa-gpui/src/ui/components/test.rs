use gpui::{
    div, impl_internal_actions, prelude::FluentBuilder, relative, App, AppContext, Context, Entity,
    IntoElement, ParentElement, Render, SharedString, Styled, Window,
};
use gpui_component::{
    blue_500,
    button::Button,
    h_flex,
    popup_menu::PopupMenuExt,
    sidebar::{Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
    v_flex, white, ActiveTheme, ContextModal, Icon, IconName,
};
use serde::Deserialize;
use tracing::info;

use crate::states::settings::SettingsTrait;

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct SelectCompany(SharedString);

impl_internal_actions!(sidebar_story, [SelectCompany]);

pub struct TestView {
    collapsed: bool,
}

impl TestView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self { collapsed: false }
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
        h_flex()
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(cx.theme().border)
            .h_full()
            .child(
                Sidebar::left()
                    .collapsed(self.collapsed)
                    .header(
                        SidebarHeader::new()
                            .w_full()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_md()
                                    .bg(blue_500())
                                    .text_color(white())
                                    .size_8()
                                    .flex_shrink_0()
                                    .when(!self.collapsed, |this| {
                                        this.child(Icon::new(IconName::GalleryVerticalEnd))
                                    })
                                    .when(self.collapsed, |this| {
                                        this.size_4()
                                            .bg(cx.theme().transparent)
                                            .text_color(cx.theme().foreground)
                                            .child(Icon::new(IconName::GalleryVerticalEnd))
                                    }),
                            )
                            .when(!self.collapsed, |this| {
                                this.child(
                                    v_flex()
                                        .gap_0()
                                        .text_sm()
                                        .flex_1()
                                        .line_height(relative(1.25))
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child("Company Name")
                                        .child(div().child("Enterprise").text_xs()),
                                )
                            })
                            .when(!self.collapsed, |this| {
                                this.child(Icon::new(IconName::ChevronsUpDown).size_4().flex_shrink_0())
                            })
                            .popup_menu(|menu, _, _| {
                                menu.menu(
                                    "Twitter Inc.",
                                    Box::new(SelectCompany(SharedString::from("twitter"))),
                                )
                                .menu(
                                    "Meta Platforms",
                                    Box::new(SelectCompany(SharedString::from("meta"))),
                                )
                                .menu(
                                    "Google Inc.",
                                    Box::new(SelectCompany(SharedString::from("google"))),
                                )
                            }),
                    )
                    .child(
                        SidebarGroup::new("Group 1").child(SidebarMenu::new().children(vec![
                            SidebarMenuItem::new("Item 1").icon(IconName::GitHub),
                            SidebarMenuItem::new("Item 2").icon(IconName::GitHub),
                            SidebarMenuItem::new("Item 3").icon(IconName::GitHub),
                            SidebarMenuItem::new("Item 4").icon(IconName::GitHub),
                            SidebarMenuItem::new("Item 5").icon(IconName::GitHub),
                            SidebarMenuItem::new("Item 6").icon(IconName::GitHub),
                            SidebarMenuItem::new("Item 7").icon(IconName::GitHub),
                            SidebarMenuItem::new("Item 8").icon(IconName::GitHub),
                        ])),
                    )
                    .footer(
                        SidebarFooter::new()
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(IconName::CircleUser)
                                    .when(!self.collapsed, |this| this.child("Jason Lee")),
                            )
                            .when(!self.collapsed, |this| {
                                this.child(Icon::new(IconName::ChevronsUpDown).size_4())
                            }),
                    ),
            )
            .child(
                v_flex()
                    .size_full()
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
