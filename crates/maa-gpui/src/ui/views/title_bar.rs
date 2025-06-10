use gpui::{
    div, Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, SharedString, Styled,
    Window,
};
use gpui_component::{
    badge::Badge,
    button::{Button, ButtonVariants},
    ContextModal, IconName, Sizable, TitleBar,
};

pub struct AppTitleBar {
    title: SharedString,
}

impl AppTitleBar {
    pub fn new(title: impl Into<SharedString>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { title: title.into() }
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notifications_count = window.notifications(cx).len();

        TitleBar::new()
            // left side
            .child(div().flex().items_center().child(self.title.clone()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(
                        Button::new("github")
                            .icon(IconName::GitHub)
                            .small()
                            .ghost()
                            .on_click(|_, _, cx| cx.open_url("https://github.com/longbridge/gpui-component")),
                    )
                    .child(
                        div().relative().child(
                            Badge::new()
                                .count(notifications_count)
                                .max(99)
                                .child(Button::new("bell").small().ghost().compact().icon(IconName::Bell)),
                        ),
                    ),
            )
    }
}
