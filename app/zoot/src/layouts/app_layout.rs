use gpui::{div, AnyElement, App, Empty, IntoElement, ParentElement, RenderOnce, Styled, Window};
use gpui_component::v_flex;

use crate::components::{tab_bar::AppTabBar, title_bar::AppTitleBar};

#[derive(IntoElement)]
pub struct AppLayout {
    pub(crate) view: AnyElement,
}

impl Default for AppLayout {
    fn default() -> Self {
        AppLayout {
            view: Empty {}.into_any_element(),
        }
    }
}

impl AppLayout {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn view(self, view: impl IntoElement) -> Self {
        Self {
            view: view.into_any_element(),
        }
    }
}

impl From<AnyElement> for AppLayout {
    fn from(view: AnyElement) -> Self {
        AppLayout { view }
    }
}

impl RenderOnce for AppLayout {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().size_full().relative().child(
            v_flex().size_full().child(AppTitleBar::new()).child(
                v_flex()
                    .flex_1()
                    .size_full()
                    .child(div().flex_1().size_full().child(self.view))
                    .child(AppTabBar::new()),
            ),
        )
    }
}
