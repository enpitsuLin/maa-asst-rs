use crate::constants::APP_NAME;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::path::PathBuf;
use std::sync::OnceLock;

/// Returns the path to the user's home directory.
pub fn home_dir() -> &'static PathBuf {
    static HOME_DIR: OnceLock<PathBuf> = OnceLock::new();
    HOME_DIR.get_or_init(|| {
        UserDirs::new()
            .expect("failed to determine home directory")
            .home_dir()
            .to_path_buf()
    })
}

/// Returns the path to the configuration directory used by MAA.
pub fn config_dir() -> &'static PathBuf {
    static CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();
    CONFIG_DIR.get_or_init(|| {
        if cfg!(target_os = "windows") {
            return BaseDirs::new()
                .expect("failed to determine RoamingAppData directory")
                .config_dir()
                .join(APP_NAME);
        }

        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            return if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
                PathBuf::from(flatpak_xdg_config)
            } else {
                BaseDirs::new()
                    .expect("failed to determine XDG_CONFIG_HOME directory")
                    .config_dir()
                    .to_path_buf()
            }
            .join(APP_NAME);
        }

        home_dir().join(".config").join(APP_NAME)
    })
}

/// Returns the path to the support directory used by MAA.
pub fn project_dir() -> &'static ProjectDirs {
    static PROJECT_DIR: OnceLock<ProjectDirs> = OnceLock::new();
    PROJECT_DIR.get_or_init(|| {
        ProjectDirs::from("me", "enpitsulin", "zoot-maa").expect("couldn't find project dirs")
    })
}
