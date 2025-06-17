use crate::paths::support_dir;
use std::{fs, sync::OnceLock};

pub mod constants;
pub mod paths;

/// Global application state containing Nostr client and shared resources
pub struct Globals {
    /// Determines if this is the first time user run MAA
    pub first_run: bool,
}

/// Global singleton instance for application state
static GLOBALS: OnceLock<Globals> = OnceLock::new();

pub fn shared_state() -> &'static Globals {
    GLOBALS.get_or_init(|| {
        let first_run = is_first_run().unwrap_or(true);

        Globals { first_run }
    })
}

impl Globals {
    pub fn read_settings(&self) -> Option<String> {
        let file = support_dir().join("zoot.json");

        if file.exists() {
            Some(fs::read_to_string(file).unwrap())
        } else {
            None
        }
    }

    pub fn write_settings(&self, settings: String) -> Result<(), anyhow::Error> {
        let file = support_dir().join("zoot.json");
        fs::write(file, settings)?;
        Ok(())
    }
}

fn is_first_run() -> Result<bool, anyhow::Error> {
    let flag = support_dir().join(".zoot_first_run");

    if !flag.exists() {
        fs::write(&flag, "")?;
        Ok(true)
    } else {
        Ok(false)
    }
}
