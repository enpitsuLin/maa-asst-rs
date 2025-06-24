use gpui::{div, App, IntoElement, ParentElement, RenderOnce, Styled, Window};
use gpui_component::{tab::TabBar, v_flex};
use route::{AppRoute, Route, ToolsSubRoute};

#[derive(IntoElement)]
pub struct ToolsLayout {
    route: ToolsSubRoute,
}

impl Default for ToolsLayout {
    fn default() -> Self {
        ToolsLayout {
            route: ToolsSubRoute::Copilot,
        }
    }
}

impl From<ToolsSubRoute> for ToolsLayout {
    fn from(route: ToolsSubRoute) -> Self {
        ToolsLayout { route }
    }
}

impl RenderOnce for ToolsLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let route = AppRoute::get_global(cx).route;

        let index = match route {
            Route::Tools(ToolsSubRoute::Copilot) => 0,
            Route::Tools(ToolsSubRoute::Recruit) => 1,
            Route::Tools(ToolsSubRoute::Gacha) => 2,
            _ => panic!("Invalid route"),
        };

        v_flex()
            .size_full()
            .relative()
            .child(
                TabBar::new("tools-tab")
                    .selected_index(index)
                    .child(ToolsSubRoute::Copilot)
                    .child(ToolsSubRoute::Recruit)
                    .child(ToolsSubRoute::Gacha)
                    .on_click(|index, _, cx| {
                        let new_route = match index {
                            0 => ToolsSubRoute::Copilot,
                            1 => ToolsSubRoute::Recruit,
                            2 => ToolsSubRoute::Gacha,
                            _ => panic!("Invalid index"),
                        };
                        let global = AppRoute::global_mut(cx);
                        global.update(cx, move |this, cx| {
                            this.route = Route::Tools(new_route);
                            cx.notify();
                        });
                    }),
            )
            .child(div().child(match self.route {
                ToolsSubRoute::Copilot => div().child("Copilot"),
                ToolsSubRoute::Recruit => div().child("Recruit"),
                ToolsSubRoute::Gacha => div().child("Gacha"),
            }))
    }
}
