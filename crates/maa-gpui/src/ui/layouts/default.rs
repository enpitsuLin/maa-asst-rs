use gpui::{div, impl_internal_actions, relative, white, App, SharedString, Window};
use gpui::{prelude::*, ElementId};
use gpui_component::breadcrumb::{Breadcrumb, BreadcrumbItem};
use gpui_component::divider::Divider;
use gpui_component::popup_menu::PopupMenuExt;
use gpui_component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarToggleButton,
};
use gpui_component::{blue_500, h_flex, v_flex, ActiveTheme, Icon, IconName};
use gpui_router::{use_location, use_navigate, IntoLayout, Outlet};
use serde::Deserialize;

use crate::states::app::AppStateTrait;

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct SelectCompany(SharedString);

impl_internal_actions!(sidebar_story, [SelectCompany]);

#[derive(IntoElement, IntoLayout)]
pub struct DefaultLayout {
    outlet: Outlet,
}

impl DefaultLayout {
    pub fn new() -> Self {
        Self {
            outlet: Outlet::new(),
        }
    }

    fn navigate(cx: &mut App, to: impl Into<SharedString>) {
        let mut navigate = use_navigate(cx);
        navigate(to.into());
    }

    fn render_sidebar(&self, cx: &mut App) -> impl IntoElement {
        let location = use_location(cx);

        let path = location.pathname.clone();

        Sidebar::left()
            .collapsed(cx.sidebar_collapsed())
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
                            .when(!cx.sidebar_collapsed(), |this| {
                                this.child(Icon::new(IconName::GalleryVerticalEnd))
                            })
                            .when(cx.sidebar_collapsed(), |this| {
                                this.size_4()
                                    .bg(cx.theme().transparent)
                                    .text_color(cx.theme().foreground)
                                    .child(Icon::new(IconName::GalleryVerticalEnd))
                            }),
                    )
                    .when(!cx.sidebar_collapsed(), |this| {
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
                    .when(!cx.sidebar_collapsed(), |this| {
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
                SidebarGroup::new("开始").child(
                    SidebarMenu::new().children([
                        SidebarMenuItem::new("首页")
                            .icon(IconName::SquareTerminal)
                            .active(path == "/")
                            .on_click(|_e, _window, cx| {
                                Self::navigate(cx, "/");
                            }),
                        SidebarMenuItem::new("一键长草")
                            .icon(IconName::SquareTerminal)
                            .active(path == "/start")
                            .on_click(|_e, _window, cx| {
                                Self::navigate(cx, "/start");
                            }),
                        SidebarMenuItem::new("自动战斗")
                            .active(path == "/copilot")
                            .icon(IconName::Bot)
                            .on_click(|_e, _window, cx| {
                                Self::navigate(cx, "/copilot");
                            }),
                    ]),
                ),
            )
            .child(
                SidebarGroup::new("平台").child(
                    SidebarMenu::new().children([
                        SidebarMenuItem::new("小工具")
                            .icon(IconName::BookOpen)
                            .active(path.starts_with("/tools"))
                            .children(vec![
                                SidebarMenuItem::new("公招识别")
                                    .active(path == "/tools/recruitment")
                                    .on_click(|_e, _window, cx| {
                                        Self::navigate(cx, "/tools/recruitment");
                                    }),
                                SidebarMenuItem::new("干员扫描")
                                    .active(path == "/tools/operators_scan")
                                    .on_click(|_e, _window, cx| {
                                        Self::navigate(cx, "/tools/operators_scan");
                                    }),
                                SidebarMenuItem::new("仓库扫描")
                                    .active(path == "/tools/depot_scan")
                                    .on_click(|_e, _window, cx| {
                                        Self::navigate(cx, "/tools/depot_scan");
                                    }),
                                SidebarMenuItem::new("视频识别")
                                    .active(path == "/tools/video_recognition")
                                    .on_click(|_e, _window, cx| {
                                        Self::navigate(cx, "/tools/video_recognition");
                                    }),
                                SidebarMenuItem::new("牛牛抽卡")
                                    .active(path == "/tools/gacha")
                                    .on_click(|_e, _window, cx| {
                                        Self::navigate(cx, "/tools/gacha");
                                    }),
                                SidebarMenuItem::new("牛牛监控")
                                    .active(path == "/tools/peep")
                                    .on_click(|_e, _window, cx| {
                                        Self::navigate(cx, "/tools/peep");
                                    }),
                            ])
                            .on_click(|_e, _window, cx| {
                                Self::navigate(cx, "/tools/recruitment");
                            }),
                        SidebarMenuItem::new("设置")
                            .active(path == "/settings")
                            .icon(IconName::Settings2)
                            .on_click(|_e, _window, cx| {
                                Self::navigate(cx, "/settings");
                            }),
                    ]),
                ),
            )
            .footer(
                SidebarFooter::new()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(IconName::CircleUser)
                            .when(!cx.sidebar_collapsed(), |this| this.child("Jason Lee")),
                    )
                    .when(!cx.sidebar_collapsed(), |this| {
                        this.child(Icon::new(IconName::ChevronsUpDown).size_4())
                    }),
            )
    }

    fn render_breadcrumb(&self, _cx: &mut App) -> impl IntoElement {
        let location = use_location(_cx);

        let path = location
            .pathname
            .split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        Breadcrumb::new()
            .item(BreadcrumbItem::new("0", "首页"))
            .when(path.len() > 0, |this| {
                path.iter().enumerate().fold(this, |breadcrumb, (index, item)| {
                    breadcrumb.item(BreadcrumbItem::new(
                        ElementId::Integer(index as u64),
                        item.to_string(),
                    ))
                })
            })
    }
}

impl RenderOnce for DefaultLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .rounded(cx.theme().radius)
            .h_full()
            .child(self.render_sidebar(cx))
            .child(
                v_flex()
                    .size_full()
                    .gap_4()
                    .p_4()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .child(
                                SidebarToggleButton::left()
                                    .collapsed(cx.sidebar_collapsed())
                                    .on_click(|_e, _window, cx| {
                                        cx.toggle_sidebar();
                                    }),
                            )
                            .child(Divider::vertical().h_4())
                            .child(self.render_breadcrumb(cx)),
                    )
                    .child(self.outlet),
            )
    }
}
