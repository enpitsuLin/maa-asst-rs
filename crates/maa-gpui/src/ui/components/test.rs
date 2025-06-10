use std::collections::HashMap;

use gpui::{
    div, impl_internal_actions, prelude::FluentBuilder, relative, App, AppContext, ClickEvent, Context,
    Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window,
};
use gpui_component::{
    badge::Badge,
    blue_500,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::Button,
    divider::Divider,
    h_flex,
    popup_menu::PopupMenuExt,
    sidebar::{
        Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
        SidebarToggleButton,
    },
    switch::Switch,
    v_flex, white, ActiveTheme, ContextModal, Icon, IconName, Sizable,
};
use serde::Deserialize;
use tracing::info;

use crate::states::{app::AppStateTrait, settings::SettingsTrait};

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct SelectCompany(SharedString);

impl_internal_actions!(sidebar_story, [SelectCompany]);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    Playground,
    Models,
    Documentation,
    Settings,
    DesignEngineering,
    SalesAndMarketing,
    Travel,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SubItem {
    History,
    Starred,
    General,
    Team,
    Billing,
    Limits,
    Settings,
    Genesis,
    Explorer,
    Quantum,
    Introduction,
    GetStarted,
    Tutorial,
    Changelog,
}

impl Item {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Playground => "Playground",
            Self::Models => "Models",
            Self::Documentation => "Documentation",
            Self::Settings => "Settings",
            Self::DesignEngineering => "Design Engineering",
            Self::SalesAndMarketing => "Sales and Marketing",
            Self::Travel => "Travel",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::Playground => IconName::SquareTerminal,
            Self::Models => IconName::Bot,
            Self::Documentation => IconName::BookOpen,
            Self::Settings => IconName::Settings2,
            Self::DesignEngineering => IconName::Frame,
            Self::SalesAndMarketing => IconName::ChartPie,
            Self::Travel => IconName::Map,
        }
    }

    pub fn handler(
        &self,
    ) -> impl Fn(&mut TestView, &ClickEvent, &mut Window, &mut Context<TestView>) + 'static {
        let item = *self;
        move |this, _, _, cx| {
            if this.active_items.contains_key(&item) {
                this.active_items.remove(&item);
            } else {
                this.active_items.insert(item, true);
            }

            this.last_active_item = item;
            cx.notify();
        }
    }

    pub fn items(&self) -> Vec<SubItem> {
        match self {
            Self::Playground => vec![SubItem::History, SubItem::Starred, SubItem::Settings],
            Self::Models => vec![SubItem::Genesis, SubItem::Explorer, SubItem::Quantum],
            Self::Documentation => vec![
                SubItem::Introduction,
                SubItem::GetStarted,
                SubItem::Tutorial,
                SubItem::Changelog,
            ],
            Self::Settings => vec![SubItem::General, SubItem::Team, SubItem::Billing, SubItem::Limits],
            _ => Vec::new(),
        }
    }
}

impl SubItem {
    pub fn label(&self) -> &'static str {
        match self {
            Self::History => "History",
            Self::Starred => "Starred",
            Self::Settings => "Settings",
            Self::Genesis => "Genesis",
            Self::Explorer => "Explorer",
            Self::Quantum => "Quantum",
            Self::Introduction => "Introduction",
            Self::GetStarted => "Get Started",
            Self::Tutorial => "Tutorial",
            Self::Changelog => "Changelog",
            Self::Team => "Team",
            Self::Billing => "Billing",
            Self::Limits => "Limits",
            Self::General => "General",
        }
    }

    pub fn handler(
        &self,
        item: &Item,
    ) -> impl Fn(&mut TestView, &ClickEvent, &mut Window, &mut Context<TestView>) + 'static {
        let item = *item;
        let subitem = *self;
        move |this, _, _, cx| {
            println!("Clicked on item: {}, child: {}", item.label(), subitem.label());
            this.active_items.insert(item, true);
            this.last_active_item = item;
            this.active_subitem = Some(subitem);
            cx.notify();
        }
    }
}

pub struct TestView {
    active_items: HashMap<Item, bool>,
    last_active_item: Item,
    active_subitem: Option<SubItem>,
    checked: bool,
}

impl TestView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self {
            active_items: HashMap::new(),
            last_active_item: Item::Playground,
            active_subitem: None,
            checked: false,
        }
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

    fn render_content(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            )
    }
}

impl Render for TestView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let groups: [Vec<Item>; 2] = [
            vec![
                Item::Playground,
                Item::Models,
                Item::Documentation,
                Item::Settings,
            ],
            vec![Item::DesignEngineering, Item::SalesAndMarketing, Item::Travel],
        ];

        h_flex()
            .rounded(cx.theme().radius)
            .h_full()
            .child(
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
                    .child(SidebarGroup::new("Platform").child(SidebarMenu::new().children(
                        groups[0].iter().map(|item| {
                            SidebarMenuItem::new(item.label())
                                .icon(item.icon())
                                .active(self.active_items.contains_key(item))
                                .children(item.items().into_iter().enumerate().map(|(ix, sub_item)| {
                                    SidebarMenuItem::new(sub_item.label())
                                        .active(self.active_subitem == Some(sub_item))
                                        .when(ix == 0, |this| {
                                            this.suffix(
                                                Switch::new("switch")
                                                    .xsmall()
                                                    .checked(self.checked)
                                                    .on_click(cx.listener(|this, checked, _, _| {
                                                        this.checked = *checked
                                                    })),
                                            )
                                        })
                                        .on_click(cx.listener(sub_item.handler(&item)))
                                }))
                                .on_click(cx.listener(item.handler()))
                        }),
                    )))
                    .child(SidebarGroup::new("Projects").child(SidebarMenu::new().children(
                        groups[1].iter().enumerate().map(|(ix, item)| {
                            SidebarMenuItem::new(item.label())
                                .icon(item.icon())
                                .active(self.last_active_item == *item)
                                .when(ix == 0, |this| {
                                    this.suffix(
                                        Badge::new()
                                            .dot()
                                            .count(1)
                                            .child(div().p_0p5().child(Icon::new(IconName::Bell))),
                                    )
                                })
                                .when(ix == 1, |this| this.suffix(IconName::Settings2))
                        }),
                    )))
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
                    ),
            )
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
                                    .on_click(cx.listener(|_, _, _, cx| {
                                        cx.toggle_sidebar();
                                        cx.notify();
                                    })),
                            )
                            .child(Divider::vertical().h_4())
                            .child(
                                Breadcrumb::new()
                                    .item(BreadcrumbItem::new("0", "Home").on_click(cx.listener(
                                        |this, _, _, cx| {
                                            this.last_active_item = Item::Playground;
                                            cx.notify();
                                        },
                                    )))
                                    .item(BreadcrumbItem::new("1", self.last_active_item.label()).on_click(
                                        cx.listener(|this, _, _, cx| {
                                            this.active_subitem = None;
                                            cx.notify();
                                        }),
                                    ))
                                    .when_some(self.active_subitem, |this, subitem| {
                                        this.item(BreadcrumbItem::new("2", subitem.label()))
                                    }),
                            ),
                    )
                    .child(self.render_content(window, cx)),
            )
    }
}
