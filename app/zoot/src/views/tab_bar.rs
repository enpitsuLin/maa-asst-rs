use gpui::{
    div, px, App, IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Styled, Window,
};
use gpui_component::{
    button::{Button, ButtonCustomVariant, ButtonRounded, ButtonVariants},
    h_flex, v_flex, ActiveTheme, Icon, Selectable, Sizable,
};

use route::{AppRoute, Route, SettingsSubRoute};

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

        let view = div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .child(format!("Hello, {}!", route.content()));

        v_flex().flex_1().size_full().child(view).child(
            h_flex()
                .h(TAB_BAR_HEIGHT)
                .w_full()
                .bg(cx.theme().accordion)
                .justify_between()
                .items_center()
                .px_4()
                .child(h_flex().gap_2().children(vec![
                    TabBarLink::new(Route::Home),
                    TabBarLink::new(Route::Copliot),
                    TabBarLink::new(Route::Tasks),
                    TabBarLink::new(Route::Dashboard),
                    TabBarLink::new(Route::Settings(SettingsSubRoute::General)),
                ]))
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
                        .rounded(ButtonRounded::Size(px(9999.))),
                ),
        )
    }
}

#[derive(IntoElement)]
struct TabBarLink {
    id: SharedString,
    route: Route,
}

impl TabBarLink {
    pub fn new(route: Route) -> Self {
        Self {
            id: route.id(),
            route,
        }
    }
}

impl RenderOnce for TabBarLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let tab_button_variant = ButtonCustomVariant::new(cx)
            .color(cx.theme().primary)
            .foreground(cx.theme().primary_foreground)
            .border(cx.theme().border)
            .shadow(cx.theme().shadow)
            .hover(cx.theme().primary_hover)
            .active(cx.theme().primary);

        let is_selected = self.route == AppRoute::get_global(cx).route;

        Button::new(self.id.clone())
            .custom(tab_button_variant)
            .label(self.route.label())
            .ghost()
            .small()
            .selected(is_selected)
            .on_click(move |_this, _, cx| {
                let global = AppRoute::global_mut(cx);
                global.update(cx, |this, cx| {
                    this.route = self.route;
                    cx.notify();
                });
            })
    }
}
