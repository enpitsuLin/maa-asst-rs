mod download;
mod extract;
mod version_json;

use std::{
    borrow::Cow,
    env::consts::{ARCH, DLL_PREFIX, DLL_SUFFIX, OS},
    fs::{create_dir, remove_dir_all},
    path::{self, Path},
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use global::paths::project_dir;
use semver::Version;
use serde::Deserialize;
use version_json::VersionJSON;

use crate::{
    download::{check_file_exists, download_mirrors},
    extract::Archive,
};

use maa_sys::Assistant;

#[derive(Deserialize)]
pub struct Details {
    assets: Vec<Asset>,
}

impl Details {
    pub fn asset(&self, name: &str) -> Result<&Asset> {
        self.assets
            .iter()
            .find(|asset| name == asset.name())
            .ok_or_else(|| anyhow!("Asset not found"))
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub struct Asset {
    name: String,
    size: u64,
    browser_download_url: String,
    mirrors: Vec<String>,
}

impl Asset {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn download_links(&self) -> Vec<String> {
        let mut links = self.mirrors.clone();
        links.insert(0, self.browser_download_url.clone());
        links
    }
}

pub enum Channel {
    Stable,
    Beta,
    Alpha,
}

static DEFAULT_API_URL: &str =
    "https://github.com/MaaAssistantArknights/MaaRelease/raw/main/MaaAssistantArknights/api/version/";

pub struct Installer {
    channel: Channel,
    api_url: Option<String>,
}

impl Installer {
    pub fn stable() -> Self {
        Self::new(Channel::Stable, None::<String>)
    }

    pub fn beta() -> Self {
        Self::new(Channel::Beta, None::<String>)
    }

    pub fn alpha() -> Self {
        Self::new(Channel::Alpha, None::<String>)
    }

    pub fn new<S: Into<String>>(channel: Channel, api_url: Option<S>) -> Self {
        Self {
            channel,
            api_url: api_url.map(|s| s.into()),
        }
    }

    fn api_url(&self) -> String {
        self.api_url.clone().unwrap_or(DEFAULT_API_URL.into())
    }

    pub fn is_installed(&self) -> bool {
        let project_dir = project_dir();
        let lib_dir = project_dir.data_dir().join("libraray");
        let resource_dir = project_dir.data_dir();
        lib_dir.exists() && resource_dir.exists()
    }

    pub(crate) fn load_current_version(&self) -> Result<Version> {
        let project_dir = project_dir();

        let lib_dir = project_dir.data_dir().join("libraray");
        let resource_dir = project_dir.data_dir(); //.join("resource");

        if !self.is_installed() {
            return Err(anyhow!("MaaCore is not installed"));
        }

        let assistant = Assistant::registry()
            .with_library(&lib_dir)
            .with_resource(&resource_dir)
            .init()?;

        let v_str = assistant.version()?;
        let v_str = v_str.trim();

        v_str
            .strip_prefix("v")
            .unwrap_or(v_str)
            .parse()
            .context("Failed to get version")
    }

    pub(crate) async fn get_version_json(&self) -> Result<VersionJSON<Details>> {
        let channel = match self.channel {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Alpha => "alpha",
        };
        let url = format!("{}/{}.json", self.api_url(), channel);
        let version_json = reqwest::get(url).await?.json::<VersionJSON<Details>>().await?;

        Ok(version_json)
    }

    pub(crate) async fn download<'p>(
        path: Cow<'p, Path>,
        size: u64,
        links: Vec<String>,
    ) -> Result<Archive<'p>> {
        if check_file_exists(&path, size) {
            tracing::info!("Already downloaded, skip downloading");
            return Archive::new(path);
        }

        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .build()
            .context("Failed to build reqwest client")?;

        download_mirrors(&client, links, &path, size, 3).await?;

        Archive::new(path)
    }

    pub(crate) fn extract_mapper(
        src: Cow<Path>,
        lib_dir: &Path,
        resource_dir: &Path,
    ) -> Option<std::path::PathBuf> {
        tracing::debug!("Extracting file: {}", src.display());
        let mut path_components = src.components();
        for c in path_components.by_ref() {
            match c {
                path::Component::Normal(c) => {
                    if c == "resource" {
                        // The components.as_path() is not working
                        // because it return a path with / as separator on windows
                        // I don't know why
                        let mut dest = resource_dir.to_path_buf();
                        for c in path_components.by_ref() {
                            dest.push(c);
                        }
                        tracing::debug!("Extracting {} => {}", src.display(), dest.display());
                        return Some(dest);
                    }
                    if c.to_str() // The DLL suffix may not the last part of the file name
                        .is_some_and(|s| s.starts_with(DLL_PREFIX) && s.contains(DLL_SUFFIX))
                    {
                        let dest = lib_dir.join(src.file_name()?);
                        tracing::debug!("Extracting {} => {}", src.display(), dest.display());
                        return Some(dest);
                    }
                },
                _ => continue,
            }
        }
        tracing::debug!("Ignored file {}", src.display());
        None
    }

    pub(crate) async fn download_archive<'p>(
        &self,
        version_json: VersionJSON<Details>,
    ) -> Result<Archive<'p>> {
        let asset_version = version_json.version();
        let asset_name = name(asset_version)?;
        let asset = version_json.details().asset(&asset_name)?;

        tracing::info!("Downloading MAA Core v{}", asset_version);

        let cache_dir = project_dir().cache_dir();
        cache_dir.ensure()?;

        Self::download(
            cache_dir.join(asset_name).into(),
            asset.size(),
            asset.download_links(),
        )
        .await
    }

    pub async fn install(&self) -> Result<()> {
        if self.is_installed() {
            tracing::error!("MaaCore already exists, please update it");
            return Ok(());
        }

        let version_json = self.get_version_json().await?;

        let archive = self.download_archive(version_json).await?;

        tracing::info!("Installing MaaCore...");
        let lib_dir = project_dir().data_dir().join("libraray");
        let resource_dir = project_dir().data_dir().join("resource");

        tracing::debug!("Cleaning library directory");
        lib_dir.ensure_clean()?;
        archive.extract(|path| Self::extract_mapper(path, &lib_dir, &resource_dir))?;

        Ok(())
    }

    pub async fn update(&self) -> Result<()> {
        let version_json = self.get_version_json().await?;
        let asset_version = version_json.version().clone();
        let current_version = self.load_current_version()?;

        if !version_json.can_update("MaaCore", &current_version)? {
            tracing::info!("MaaCore is up to date");
            return Ok(());
        }

        let archive = self.download_archive(version_json).await?;

        tracing::info!("Updating MaaCore  to v{}...", asset_version);
        let lib_dir = project_dir().data_dir().join("libraray");
        let resource_dir = project_dir().data_dir().join("resource");

        tracing::debug!("Cleaning library directory");
        lib_dir.ensure_clean()?;
        archive.extract(|path| Self::extract_mapper(path, &lib_dir, &resource_dir))?;

        Ok(())
    }
}

trait Ensure: Sized {
    fn ensure(self) -> Result<Self>;
    fn ensure_clean(self) -> Result<Self>;
}

impl Ensure for &Path {
    fn ensure(self) -> Result<Self> {
        if !self.exists() {
            std::fs::create_dir_all(self).context("Failed to create directory")?;
        }
        Ok(self)
    }

    fn ensure_clean(self) -> Result<Self> {
        if self.exists() {
            let mut ret = remove_dir_all(self);
            for i in 1..=3 {
                if let Err(err) = &ret {
                    tracing::warn!(
                        "Failed to remove dir {} due to {err}, retry {i} times",
                        self.display()
                    );
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    ret = remove_dir_all(self);
                } else {
                    break;
                }
            }
            ret?;
        } else if let Some(parent) = self.parent() {
            parent.ensure()?;
        }
        create_dir(self)?;
        Ok(self)
    }
}

/// Get the name of the asset for the current platform
pub fn name(version: &Version) -> Result<String> {
    match OS {
        "macos" => Ok(format!("MAA-v{}-macos-runtime-universal.zip", version)),
        "linux" => match ARCH {
            "x86_64" => Ok(format!("MAA-v{}-linux-x86_64.tar.gz", version)),
            "aarch64" => Ok(format!("MAA-v{}-linux-aarch64.tar.gz", version)),
            _ => Err(anyhow!("Unsupported architecture: {}", ARCH)),
        },
        "windows" => match ARCH {
            "x86_64" => Ok(format!("MAA-v{}-win-x64.zip", version)),
            "aarch64" => Ok(format!("MAA-v{}-win-arm64.zip", version)),
            _ => Err(anyhow!("Unsupported architecture: {}", ARCH)),
        },
        _ => Err(anyhow!("Unsupported platform: {}", OS)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_install() {
        Installer::stable().install().await.unwrap();
    }

    #[test]
    fn test_load_current_version() {
        let installer = Installer::stable();
        let version = installer.load_current_version().unwrap();
        println!("Current version: {}", version);
    }

    #[tokio::test]
    async fn test_update() {
        Installer::beta().update().await.unwrap();
    }
}
