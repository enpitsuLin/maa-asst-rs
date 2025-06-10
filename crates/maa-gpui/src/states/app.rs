use gpui::{App, Global, SharedString};

pub struct AppState {
    pub title: SharedString,
}

impl Global for AppState {}

impl AppState {
    pub fn init(cx: &mut App, title: impl Into<SharedString>) {
        cx.set_global(Self { title: title.into() });
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
}
