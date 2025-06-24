use gpui::{div, Context, IntoElement, ParentElement, Render, Styled, Window};
use route::{AppRoute, Route, SettingsSubRoute};

use crate::layouts::{app_layout::AppLayout, tools_layout::ToolsLayout};

pub struct ZootApp {}

impl Render for ZootApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let route = AppRoute::get_global(cx).route;

        let view = match route {
            Route::Home => div().text_xs().child("Home"),
            Route::Tools(sub_route) => div().child(ToolsLayout::from(sub_route)),
            Route::Tasks => div().text_xs().child("Tasks"),
            Route::Dashboard => div().text_xs().child("Dashboard"),
            Route::Settings(sub_route) => match sub_route {
                SettingsSubRoute::General => div().text_xs().child("Settings / General"),
                SettingsSubRoute::Advanced => div().text_xs().child("Settings / Advanced"),
                SettingsSubRoute::About => div().text_xs().child("Settings / About"),
            },
        };

        AppLayout::new().view(view)
    }
}
