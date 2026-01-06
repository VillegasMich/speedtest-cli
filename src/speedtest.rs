use crate::error::{Result, SpeedTestError};
use futures_util::StreamExt;
use log::{debug, info};
use reqwest::Client;
use std::time::{Duration, Instant};

// Using multiple test file URLs for download testing
const DOWNLOAD_TEST_URLS: &[&str] = &[
    "https://speed.cloudflare.com/__down?bytes=10000000",
    "https://proof.ovh.net/files/10Mb.dat",
    "http://speedtest.ftp.otenet.gr/files/test10Mb.db",
];

const UPLOAD_TEST_URL: &str = "https://httpbin.org/post";
const UPLOAD_SIZE_MB: usize = 5;
const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_millis(100);

pub type ProgressCallback = Box<dyn Fn(f64) + Send + Sync>;

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

    #[allow(dead_code)]
    pub async fn test_download(&self) -> Result<f64> {
        self.test_download_with_progress(None, 30).await
    }

    pub async fn test_download_with_progress(
        &self,
        progress_callback: Option<ProgressCallback>,
        duration_secs: u64,
    ) -> Result<f64> {
        info!("Starting download speed test with {} second duration...", duration_secs);

        // Try multiple URLs until one works
        let mut last_error = None;
        for url in DOWNLOAD_TEST_URLS {
            debug!("Attempting download from: {}", url);

            match self.try_download(url, progress_callback.as_ref(), duration_secs).await {
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

    async fn try_download(
        &self,
        url: &str,
        progress_callback: Option<&ProgressCallback>,
        duration_secs: u64,
    ) -> Result<f64> {
        let start = Instant::now();
        let max_duration = Duration::from_secs(duration_secs);
        let mut total_bytes = 0u64;
        let mut last_update = Instant::now();
        let mut download_count = 0;

        // Keep downloading until duration is reached
        while start.elapsed() < max_duration {
            download_count += 1;
            debug!("Starting download iteration {}", download_count);

            let response = self.client.get(url).send().await.map_err(|e| {
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

            while let Some(chunk) = stream.next().await {
                // Check if duration has elapsed
                if start.elapsed() >= max_duration {
                    debug!("Download test duration reached, stopping");
                    break;
                }

                let chunk = chunk.map_err(|e| {
                    log::error!("Error reading download chunk: {}", e);
                    e
                })?;
                total_bytes += chunk.len() as u64;

                // Report progress if callback is provided
                if let Some(callback) = progress_callback {
                    if last_update.elapsed() >= PROGRESS_UPDATE_INTERVAL {
                        let elapsed = start.elapsed().as_secs_f64();
                        if elapsed > 0.0 {
                            let current_speed = total_bytes as f64 / elapsed;
                            callback(current_speed);
                            last_update = Instant::now();
                        }
                    }
                }
            }

            // Check again if duration has elapsed after stream ends
            if start.elapsed() >= max_duration {
                break;
            }
        }

        let elapsed = start.elapsed().as_secs_f64();
        let bytes_per_second = total_bytes as f64 / elapsed;

        debug!("Downloaded {} bytes in {:.2} seconds ({} iterations)", total_bytes, elapsed, download_count);
        info!("Download test completed: {:.2} bytes/s", bytes_per_second);

        Ok(bytes_per_second)
    }

    #[allow(dead_code)]
    pub async fn test_upload(&self) -> Result<f64> {
        self.test_upload_with_progress(None, 30).await
    }

    pub async fn test_upload_with_progress(
        &self,
        progress_callback: Option<ProgressCallback>,
        duration_secs: u64,
    ) -> Result<f64> {
        info!("Starting upload speed test with {} second duration...", duration_secs);

        let start = Instant::now();
        let max_duration = Duration::from_secs(duration_secs);
        let mut total_bytes = 0u64;
        let mut upload_count = 0;

        // Upload chunks until duration is reached
        while start.elapsed() < max_duration {
            let chunk_size = UPLOAD_SIZE_MB * 1024 * 1024;
            let data = vec![0u8; chunk_size];

            let chunk_start = Instant::now();
            let response = self
                .client
                .post(UPLOAD_TEST_URL)
                .body(data.clone())
                .send()
                .await;

            match response {
                Ok(resp) if resp.status().is_success() => {
                    total_bytes += data.len() as u64;
                    upload_count += 1;

                    // Update progress
                    if let Some(callback) = &progress_callback {
                        let elapsed = start.elapsed().as_secs_f64();
                        if elapsed > 0.0 {
                            let current_speed = total_bytes as f64 / elapsed;
                            callback(current_speed);
                        }
                    }

                    debug!("Upload chunk {} completed in {:.2}s", upload_count, chunk_start.elapsed().as_secs_f64());
                }
                Ok(resp) => {
                    log::warn!("Upload request failed with status: {}", resp.status());
                }
                Err(e) => {
                    log::warn!("Upload request error: {}", e);
                }
            }

            // Check if we've exceeded duration
            if start.elapsed() >= max_duration {
                break;
            }
        }

        let elapsed = start.elapsed().as_secs_f64();
        let bytes_per_second = total_bytes as f64 / elapsed;

        debug!("Uploaded {} bytes in {:.2} seconds ({} chunks)", total_bytes, elapsed, upload_count);
        info!("Upload test completed: {:.2} bytes/s", bytes_per_second);

        Ok(bytes_per_second)
    }

    #[allow(dead_code)]
    pub async fn test_both(&self) -> Result<(f64, f64)> {
        let download_speed = self.test_download().await?;
        let upload_speed = self.test_upload().await?;

        Ok((download_speed, upload_speed))
    }

    #[allow(dead_code)]
    pub async fn test_both_with_progress(
        &self,
        download_callback: Option<ProgressCallback>,
        upload_callback: Option<ProgressCallback>,
        duration_secs: u64,
    ) -> Result<(f64, f64)> {
        let download_speed = self.test_download_with_progress(download_callback, duration_secs).await?;
        let upload_speed = self.test_upload_with_progress(upload_callback, duration_secs).await?;

        Ok((download_speed, upload_speed))
    }
}

impl Default for SpeedTest {
    fn default() -> Self {
        Self::new()
    }
}
