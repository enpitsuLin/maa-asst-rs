use gpui::Hsla;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Preference {
    #[serde(default)]
    pub font_size: usize,
    pub theme_color: Option<Hsla>,
    pub locale: Option<String>,
}
