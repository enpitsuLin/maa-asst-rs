use anyhow::anyhow;
use global::shared_state;
use gpui::{App, AppContext, Context, Entity, Global, Subscription, Task};
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};

pub fn init(cx: &mut App) {
    let state = cx.new(AppSettings::new);

    // Observe for state changes and save settings to file
    state.update(cx, |this, cx| {
        this.subscriptions.push(cx.observe(&state, |this, _state, cx| {
            this.set_settings(cx);
        }));
    });

    AppSettings::set_global(state, cx);
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Settings {
    pub foo: String,
}

impl AsRef<Settings> for Settings {
    fn as_ref(&self) -> &Settings {
        self
    }
}

struct GlobalAppSettings(Entity<AppSettings>);

impl Global for GlobalAppSettings {}

pub struct AppSettings {
    pub settings: Settings,
    #[allow(dead_code)]
    subscriptions: SmallVec<[Subscription; 1]>,
}

impl AppSettings {
    /// Retrieve the Global Settings instance
    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalAppSettings>().0.clone()
    }

    /// Retrieve the Settings instance
    pub fn get_global(cx: &App) -> &Self {
        cx.global::<GlobalAppSettings>().0.read(cx)
    }

    /// Set the Global Settings instance
    pub(crate) fn set_global(state: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalAppSettings(state));
    }

    fn new(cx: &mut Context<Self>) -> Self {
        let settings = Settings {
            foo: "bar".to_string(),
        };

        let mut subscriptions = smallvec![];

        subscriptions.push(cx.observe_new::<Self>(|this, _window, cx| {
            this.get_settings_from_file(cx);
        }));

        Self {
            settings,
            subscriptions,
        }
    }

    pub(crate) fn get_settings_from_file(&self, cx: &mut Context<Self>) {
        let task: Task<Result<Settings, anyhow::Error>> = cx.background_spawn(async move {
            if let Some(event) = shared_state().read_settings() {
                Ok(serde_json::from_str(&event)?)
            } else {
                Err(anyhow!("Not found"))
            }
        });

        cx.spawn(async move |this, cx| match task.await {
            Ok(settings) => {
                this.update(cx, |this, cx| {
                    this.settings = settings;
                    cx.notify();
                })
                .ok();
            },
            Err(e) => {
                tracing::debug!("File {e} Not found, creating new file");
                let settings = Settings {
                    foo: "bar".to_string(),
                };
                let content = serde_json::to_string_pretty(&settings).unwrap();
                shared_state().write_settings(content).unwrap();
            },
        })
        .detach();
    }

    pub(crate) fn set_settings(&self, cx: &mut Context<Self>) {
        if let Ok(content) = serde_json::to_string_pretty(&self.settings) {
            cx.background_spawn(async move {
                if let Err(e) = shared_state().write_settings(content) {
                    tracing::error!("Failed to save user settings: {e}");
                } else {
                    tracing::info!("New settings have been saved successfully");
                }
            })
            .detach();
        }
    }
}
