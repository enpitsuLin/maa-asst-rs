use gpui::{div, px, App, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window};
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    tab::TabBar,
    v_flex, ActiveTheme, Icon, Sizable,
};

use route::{AppRoute, Route, SettingsSubRoute, ToolsSubRoute};

pub const TAB_BAR_HEIGHT: Pixels = px(64.);

#[derive(IntoElement)]
pub struct AppTabBar {}

impl AppTabBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for AppTabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let route = AppRoute::get_global(cx).route;

        let view = match route {
            Route::Home => div().text_xs().child("Home"),
            Route::Tools(sub_route) => match sub_route {
                ToolsSubRoute::Copliot => div().text_xs().child("Tools / Copliot"),
                ToolsSubRoute::Recruit => div().text_xs().child("Tools / Recruit"),
                ToolsSubRoute::Gacha => div().text_xs().child("Tools / Gacha"),
            },
            Route::Tasks => div().text_xs().child("Tasks"),
            Route::Dashboard => div().text_xs().child("Dashboard"),
            Route::Settings(sub_route) => match sub_route {
                SettingsSubRoute::General => div().text_xs().child("Settings / General"),
                SettingsSubRoute::Advanced => div().text_xs().child("Settings / Advanced"),
                SettingsSubRoute::About => div().text_xs().child("Settings / About"),
            },
        };

        let index = match route {
            Route::Home => 0,
            Route::Tools(_) => 1,
            Route::Tasks => 2,
            Route::Dashboard => 3,
            Route::Settings(_) => 4,
        };

        v_flex()
            .flex_1()
            .size_full()
            .child(div().flex_1().size_full().child(view))
            .child(
                h_flex()
                    .h(TAB_BAR_HEIGHT)
                    .w_full()
                    .bg(cx.theme().accordion)
                    .justify_between()
                    .items_center()
                    .px_4()
                    .child(
                        TabBar::new("pill")
                            .w_full()
                            .pill()
                            .selected_index(index)
                            .child(Route::Home)
                            .child(Route::Tools(ToolsSubRoute::Copliot))
                            .child(Route::Tasks)
                            .child(Route::Dashboard)
                            .child(Route::Settings(SettingsSubRoute::General))
                            .on_click(|index: &usize, _, cx| {
                                let new_route = match index {
                                    0 => Route::Home,
                                    1 => Route::Tools(ToolsSubRoute::Copliot),
                                    2 => Route::Tasks,
                                    3 => Route::Dashboard,
                                    4 => Route::Settings(SettingsSubRoute::General),
                                    _ => panic!("Invalid index"),
                                };
                                let global = AppRoute::global_mut(cx);
                                global.update(cx, move |this, cx| {
                                    this.route = new_route;
                                    cx.notify();
                                });
                            }),
                    )
                    .child(
                        Button::new("start")
                            .child(
                                h_flex()
                                    .gap_1()
                                    .items_center()
                                    .child(Icon::empty().path("icons/play.svg"))
                                    .child(div().py_1().text_xs().child("一键长草")),
                            )
                            .primary()
                            .with_size(px(50.))
                            .px_4()
                            .rounded(px(9999.)),
                    ),
            )
    }
}
