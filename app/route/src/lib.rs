use gpui::{
    App, AppContext, Context, Entity, Global, SharedString,
    Subscription,
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
    Advanced,
    About,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Route {
    Home,
    Copliot,
    Tasks,
    Dashboard,
    Settings(SettingsSubRoute),
}

impl Route {
    pub fn content(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("Home"),
            Route::Copliot => SharedString::new_static("Copliot"),
            Route::Tasks => SharedString::new_static("Tasks"),
            Route::Dashboard => SharedString::new_static("Dashboard"),
            Route::Settings(sub_route) => match sub_route {
                SettingsSubRoute::General => SharedString::new_static("Settings / General"),
                SettingsSubRoute::Advanced => SharedString::new_static("Settings / Advanced"),
                SettingsSubRoute::About => SharedString::new_static("Settings / About"),
            },
        }
    }
    pub fn id(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("home"),
            Route::Copliot => SharedString::new_static("copliot"),
            Route::Tasks => SharedString::new_static("tasks"),
            Route::Dashboard => SharedString::new_static("dashboard"),
            Route::Settings(sub_route) => match sub_route {
                SettingsSubRoute::General => SharedString::new_static("settings-general"),
                SettingsSubRoute::Advanced => SharedString::new_static("settings-advanced"),
                SettingsSubRoute::About => SharedString::new_static("settings-about"),
            },
        }
    }

    pub fn label(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("主页"),
            Route::Copliot => SharedString::new_static("自动战斗"),
            Route::Tasks => SharedString::new_static("任务列表"),
            Route::Dashboard => SharedString::new_static("仪表盘"),
            Route::Settings(_) => SharedString::new_static("设置"),
        }
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
