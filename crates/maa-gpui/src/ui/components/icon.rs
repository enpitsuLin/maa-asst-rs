use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use gpui_component::Icon;

pub enum MAAIcons {
    Home,
    Terminal,
    Bot,
    Tools,
    Settings,
    Bell,
}

impl From<MAAIcons> for Icon {
    fn from(value: MAAIcons) -> Self {
        match value {
            MAAIcons::Home => Icon::default().path("icons/home.svg"),
            MAAIcons::Terminal => Icon::default().path("icons/terminal.svg"),
            MAAIcons::Bot => Icon::default().path("icons/bot.svg"),
            MAAIcons::Tools => Icon::default().path("icons/tools.svg"),
            MAAIcons::Settings => Icon::default().path("icons/settings.svg"),
            MAAIcons::Bell => Icon::default().path("icons/bell.svg"),
        }
    }
}

impl From<MAAIcons> for AnyElement {
    fn from(val: MAAIcons) -> Self {
        Icon::from(val).into()
    }
}

impl RenderOnce for MAAIcons {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        Icon::from(self)
    }
}
