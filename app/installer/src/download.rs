use reqwest::Client;
use std::{
    cmp::min,
    fs::File,
    io::Write,
    path::Path,
    time::{Duration, Instant},
};
use tokio_stream::StreamExt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => e.fmt(f),
            Error::Io(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

// download a file with given url and size to a given path,
// with optional checksum verification.
//
// # Arguments
// * `client` - A reqwest client.
// * `url` - The url to download from.
// * `path` - The path to save the downloaded file.
// * `size` - The size of the file.
// * `checker` - The optional checksum checker.
pub async fn download(client: &Client, url: &str, path: &Path, size: u64) -> Result<()> {
    let resp = client.get(url).send().await?;

    let mut stream = resp.bytes_stream();
    let mut file = File::create(path)?;

    let mut downloaded: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded = min(downloaded + chunk.len() as u64, size);
        // update progress
    }

    // progress finish

    Ok(())
}

/// Try to download a file with given url and timeout.
///
/// # Arguments
/// * `client` - A reqwest client.
/// * `url` - The url to download from.
/// * `timeout` - The timeout.
///
/// # Returns
/// The number of bytes downloaded.
async fn try_download(client: &Client, url: &str, timeout: Duration) -> Result<u64> {
    let resp = client.get(url).send().await?;

    let mut stream = resp.bytes_stream();
    let mut downloaded: u64 = 0;
    let start = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        downloaded += chunk.len() as u64;
        if start.elapsed() > timeout {
            return Ok(downloaded);
        }
    }

    Ok(downloaded)
}

/// Download from multiple mirrors and choose the fastest one.
///
/// # Arguments
/// * `client` - A reqwest client.
/// * `mirrors` - The mirrors to choose from.
/// * `path` - The path to save the downloaded file.
/// * `size` - The size of the file.
/// * `t` - The test duration for each mirror, in seconds, 0 to skip the test.
/// * `checker` - The optional checksum checker.
pub async fn download_mirrors(
    client: &Client,
    mirrors: Vec<String>,
    path: &Path,
    size: u64,
    t: u64,
) -> Result<()> {
    // The first mirror is the default download link.
    let mut download_link = &mirrors[0];

    if t == 0 {
        tracing::info!("Skip speed test, downloading from first link: {}", download_link);
        download(client, download_link, path, size).await?;
        return Ok(());
    }

    let test_duration = Duration::from_secs(t);
    let mut largest: u64 = 0;

    tracing::debug!("Testing download speed...");
    for link in mirrors.iter() {
        tracing::debug!("Testing {}", link);
        if let Ok(downloaded) = try_download(client, link, test_duration).await {
            if downloaded > largest {
                tracing::debug!("Found faster link {} with {} bytes downloaded", link, downloaded);
                download_link = link;
                largest = downloaded;
            }
        }
    }

    tracing::info!("Downloading from fastest mirror link: {}", download_link);
    download(client, download_link, path, size).await?;

    Ok(())
}

pub fn check_file_exists(path: &Path, size: u64) -> bool {
    path.exists() && path.is_file() && path.metadata().is_ok_and(|metadata| metadata.len() == size)
}
