use crate::error::{Result, SpeedTestError};
use futures_util::StreamExt;
use log::{debug, info};
use reqwest::Client;
use std::time::Instant;

// Using multiple test file URLs for download testing
const DOWNLOAD_TEST_URLS: &[&str] = &[
    "https://speed.cloudflare.com/__down?bytes=10000000",
    "https://proof.ovh.net/files/10Mb.dat",
    "http://speedtest.ftp.otenet.gr/files/test10Mb.db",
];

const UPLOAD_TEST_URL: &str = "https://httpbin.org/post";
const UPLOAD_SIZE_MB: usize = 5;

pub struct SpeedTest {
    client: Client,
}

impl SpeedTest {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn test_download(&self) -> Result<f64> {
        info!("Starting download speed test...");

        // Try multiple URLs until one works
        let mut last_error = None;
        for url in DOWNLOAD_TEST_URLS {
            debug!("Attempting download from: {}", url);

            match self.try_download(url).await {
                Ok(speed) => return Ok(speed),
                Err(e) => {
                    log::warn!("Failed to download from {}: {}", url, e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            SpeedTestError::TestFailed("All download servers failed".to_string())
        }))
    }

    async fn try_download(&self, url: &str) -> Result<f64> {
        let start = Instant::now();
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| {
                log::error!("Failed to connect to download server: {}", e);
                e
            })?;

        if !response.status().is_success() {
            return Err(SpeedTestError::TestFailed(format!(
                "Download request failed with status: {}",
                response.status()
            )));
        }

        let mut stream = response.bytes_stream();
        let mut total_bytes = 0u64;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| {
                log::error!("Error reading download chunk: {}", e);
                e
            })?;
            total_bytes += chunk.len() as u64;
        }

        let elapsed = start.elapsed().as_secs_f64();
        let bytes_per_second = total_bytes as f64 / elapsed;

        debug!(
            "Downloaded {} bytes in {:.2} seconds",
            total_bytes, elapsed
        );
        info!("Download test completed: {:.2} bytes/s", bytes_per_second);

        Ok(bytes_per_second)
    }

    pub async fn test_upload(&self) -> Result<f64> {
        info!("Starting upload speed test...");

        let data = vec![0u8; UPLOAD_SIZE_MB * 1024 * 1024];
        let data_len = data.len() as u64;

        let start = Instant::now();
        let response = self.client
            .post(UPLOAD_TEST_URL)
            .body(data)
            .send()
            .await
            .map_err(|e| {
                log::error!("Failed to connect to upload server: {}", e);
                e
            })?;

        if !response.status().is_success() {
            return Err(SpeedTestError::TestFailed(format!(
                "Upload request failed with status: {}",
                response.status()
            )));
        }

        let elapsed = start.elapsed().as_secs_f64();
        let bytes_per_second = data_len as f64 / elapsed;

        debug!("Uploaded {} bytes in {:.2} seconds", data_len, elapsed);
        info!("Upload test completed: {:.2} bytes/s", bytes_per_second);

        Ok(bytes_per_second)
    }

    pub async fn test_both(&self) -> Result<(f64, f64)> {
        let download_speed = self.test_download().await?;
        let upload_speed = self.test_upload().await?;

        Ok((download_speed, upload_speed))
    }
}

impl Default for SpeedTest {
    fn default() -> Self {
        Self::new()
    }
}
