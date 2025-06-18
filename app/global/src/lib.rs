use crate::paths::project_dir;
use maa_sys::Assistant;
use std::{
    fs,
    sync::{OnceLock, RwLock},
};

pub mod constants;
pub mod paths;

/// Global application state containing Nostr client and shared resources
pub struct Globals {
    /// Determines if this is the first time user run MAA
    pub first_run: bool,
    /// Mutable Assistant instance
    pub assistant: RwLock<Option<Assistant>>,
}

/// Global singleton instance for application state
static GLOBALS: OnceLock<Globals> = OnceLock::new();

pub fn shared_state() -> &'static Globals {
    GLOBALS.get_or_init(|| {
        let first_run = is_first_run().unwrap_or(true);

        Globals { 
            first_run,
            assistant: RwLock::new(None),
        }
    })
}

impl Globals {
    /// 设置 Assistant 实例
    pub fn set_assistant(&self, assistant: Assistant) {
        let mut current_assistant = self.assistant.write().unwrap();
        *current_assistant = Some(assistant);
    }

    /// 清除 Assistant 实例
    pub fn clear_assistant(&self) {
        let mut assistant = self.assistant.write().unwrap();
        *assistant = None;
    }

    /// 检查是否有 Assistant 实例
    pub fn has_assistant(&self) -> bool {
        let assistant = self.assistant.read().unwrap();
        assistant.is_some()
    }

    /// 使用 Assistant 执行操作（避免克隆）
    pub fn with_assistant<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Assistant) -> R,
    {
        let assistant = self.assistant.read().unwrap();
        assistant.as_ref().map(f)
    }

    /// 可变地使用 Assistant 执行操作
    pub fn with_assistant_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Assistant) -> R,
    {
        let mut assistant = self.assistant.write().unwrap();
        assistant.as_mut().map(f)
    }

    pub fn read_settings(&self) -> Option<String> {
        let file = project_dir().config_dir().join("zoot.json");

        if file.exists() {
            Some(fs::read_to_string(file).unwrap())
        } else {
            None
        }
    }

    pub fn write_settings(&self, settings: String) -> Result<(), anyhow::Error> {
        let dir = project_dir().config_dir();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let file = dir.join("zoot.json");
        fs::write(file, settings)?;
        Ok(())
    }
}

fn is_first_run() -> Result<bool, anyhow::Error> {
    let flag = project_dir().data_dir().join(".zootrc");

    if !flag.exists() {
        fs::write(&flag, "")?;
        Ok(true)
    } else {
        Ok(false)
    }
}
