use gpui::{
    div, App, AppContext, Context, Entity, Global, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Subscription, Window,
};
use smallvec::{smallvec, SmallVec};

pub fn init(cx: &mut App) {
    let state = cx.new(AppRoute::new);

    state.update(cx, |this, cx| {
        this.subscriptions.push(cx.observe(&state, |_this, _state, cx| {
            // route change side effect
        }));
    });

    AppRoute::set_global(state, cx);
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SettingsSubRoute {
    General,
}

#[derive(PartialEq, Eq, Clone, Copy, IntoElement)]
pub enum Route {
    Home,
    Tasks,
    Dashboard,
    Settings(SettingsSubRoute),
}

impl Route {
    fn content(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("Home"),
            Route::Tasks => SharedString::new_static("Tasks"),
            Route::Dashboard => SharedString::new_static("Dashboard"),
            Route::Settings(sub_route) => match sub_route {
                SettingsSubRoute::General => SharedString::new_static("Settings / General"),
            },
        }
    }
    pub fn id(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("home"),
            Route::Tasks => SharedString::new_static("tasks"),
            Route::Dashboard => SharedString::new_static("dashboard"),
            Route::Settings(sub_route) => match sub_route {
                SettingsSubRoute::General => SharedString::new_static("settings-general"),
            },
        }
    }

    pub fn label(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("主页"),
            Route::Tasks => SharedString::new_static("任务列表"),
            Route::Dashboard => SharedString::new_static("仪表盘"),
            Route::Settings(_) => SharedString::new_static("设置"),
        }
    }
}

impl RenderOnce for Route {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .child(format!("Hello, {}!", self.content()))
    }
}

struct GlobalAppRoute(Entity<AppRoute>);

impl Global for GlobalAppRoute {}

pub struct AppRoute {
    pub route: Route,
    #[allow(dead_code)]
    subscriptions: SmallVec<[Subscription; 1]>,
}

impl AppRoute {
    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalAppRoute>().0.clone()
    }

    pub fn get_global(cx: &App) -> &Self {
        cx.global::<GlobalAppRoute>().0.read(cx)
    }

    pub fn global_mut(cx: &mut App) -> Entity<Self> {
        cx.global_mut::<GlobalAppRoute>().0.clone()
    }

    pub(crate) fn set_global(state: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalAppRoute(state));
    }

    fn new(cx: &mut Context<Self>) -> Self {
        let mut subscriptions = smallvec![];

        subscriptions.push(cx.observe_new::<Self>(|_this, _window, cx| {
            // initial side effect
        }));

        Self {
            route: Route::Home,
            subscriptions,
        }
    }
}
