use gpui::{App, Global, SharedString};

pub struct AppState {
    pub title: SharedString,
    pub sidebar_collapsed: bool
}

impl Global for AppState {}

impl AppState {
    pub fn init(cx: &mut App, title: impl Into<SharedString>) {
        cx.set_global(Self {
            title: title.into(),
            sidebar_collapsed: false
        });
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

pub trait AppStateTrait {
    fn update_title(&mut self, title: impl Into<SharedString>);
    fn app_title(&self) -> SharedString;

    fn toggle_sidebar(&mut self);
    fn set_sidebar_collapsed(&mut self, collapsed: bool);
    fn sidebar_collapsed(&self) -> bool;
}

impl AppStateTrait for App {
    fn update_title(&mut self, title: impl Into<SharedString>) {
        let title = title.into();

        let global = AppState::global_mut(self);
        global.title = title;
    }

    fn app_title(&self) -> SharedString {
        AppState::global(self).title.clone()
    }

    fn toggle_sidebar(&mut self) {
        let collapsed = {
            let global = AppState::global_mut(self);
            !global.sidebar_collapsed
        };
        self.set_sidebar_collapsed(collapsed);
    }

    fn set_sidebar_collapsed(&mut self, collapsed: bool) {
        let global = AppState::global_mut(self);
        global.sidebar_collapsed = collapsed;
    }

    fn sidebar_collapsed(&self) -> bool {
        AppState::global(self).sidebar_collapsed
    }
}
