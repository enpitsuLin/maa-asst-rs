use gpui::{App, EventEmitter, Global};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};
use tracing::warn;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub adb_path: Option<String>,
}

impl EventEmitter<String> for Settings {}

impl Global for Settings {}

impl Settings {
    pub fn init(cx: &mut App, path: PathBuf) {
        let settings = Self::create_settings(&path);

        cx.set_global(settings);

        cx.observe_global::<Settings>(move |app| {
            let settings = app.global::<Settings>().clone();

            let file = File::create(&path).unwrap();
            serde_json::to_writer_pretty(file, &settings).unwrap();
        })
        .detach();
    }

    pub fn create_settings(path: &PathBuf) -> Settings {
        let Ok(file) = File::open(path) else {
            return Settings::default();
        };
        let reader = std::io::BufReader::new(file);

        if let Ok(settings) = serde_json::from_reader(reader) {
            settings
        } else {
            warn!("Failed to parse settings file, using default settings");
            Settings::default()
        }
    }
}
