use gpui::prelude::*;
use gpui::{div, App, Context, Entity, Window};
use gpui_router::{IntoLayout, Outlet, Route, Routes};

use crate::ui::layouts::default::DefaultLayout;

pub struct RouterTestView {}

impl RouterTestView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self {}
    }
}
impl Render for RouterTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            Routes::new().child(
                Route::new()
                    .layout(DefaultLayout::new())
                    .child(Route::new().index().element(view("Home")))
                    .child(Route::new().path("start").element(view("Start")))
                    .child(Route::new().path("copilot").element(view("Copilot")))
                    .child(
                        Route::new()
                            .path("tools")
                            .layout(SettingLayout::new())
                            .child(Route::new().index().element(view("Tools")))
                            .child(Route::new().path("recruitment").element(view("Recruitment")))
                            .child(Route::new().path("operators_scan").element(view("OperatorsScan")))
                            .child(Route::new().path("depot_scan").element(view("DepotScan")))
                            .child(
                                Route::new()
                                    .path("video_recognition")
                                    .element(view("VideoRecognition"))
                            )
                            .child(Route::new().path("gacha").element(view("Gacha")))
                            .child(Route::new().path("peep").element(view("Peep")))
                    )
                    .child(Route::new().path("settings").element(view("Settings")))
                    .child(Route::new().path("{*not_match}").element(view("NotMatch")))
            )
        )
    }
}

#[derive(IntoElement, IntoLayout)]
pub struct SettingLayout {
    outlet: Outlet
}

impl SettingLayout {
    pub fn new() -> Self {
        Self {
            outlet: Outlet::new()
        }
    }
}

impl RenderOnce for SettingLayout {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .child(div().flex().gap_4().text_lg().child("Setting Layout"))
            .child(self.outlet)
    }
}

fn view(name: &str) -> impl IntoElement {
    div().child(format!("View: {}", name))
}
