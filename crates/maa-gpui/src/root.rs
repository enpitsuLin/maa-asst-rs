use gpui::{div, AnyView, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{v_flex, Root};

use crate::ui::views::title_bar::AppTitleBar;

pub struct MAARoot {
    title_bar: Entity<AppTitleBar>,
    view: AnyView,
}

impl MAARoot {
    pub fn new(view: impl Into<AnyView>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let title_bar = cx.new(|_| AppTitleBar {});
        Self {
            title_bar,
            view: view.into(),
        }
    }
}

impl Render for MAARoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let drawer_layer = Root::render_drawer_layer(window, cx);
        let modal_layer = Root::render_modal_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            .children(drawer_layer)
            .children(modal_layer)
            .child(div().absolute().top_8().children(notification_layer))
    }
}
