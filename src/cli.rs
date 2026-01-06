use crate::error::Result;
use crate::output::{
    create_progress_bar, create_progress_callback, print_error, print_success, SpeedTestResult,
};
use crate::speedtest::SpeedTest;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "speedtest-cli")]
#[command(about = "A CLI tool to test internet speed", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, value_enum, default_value_t = SpeedUnit::Mbps, global = true)]
    pub unit: SpeedUnit,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

impl Cli {
    pub async fn execute(&self) -> Result<()> {
        let speed_test = SpeedTest::new();
        let result = match self.command {
            Commands::Start => run_full_test(&speed_test, self.unit).await,
            Commands::Download => run_download_test(&speed_test, self.unit).await,
            Commands::Upload => run_upload_test(&speed_test, self.unit).await,
        };

        match result {
            Ok(test_result) => {
                print_success(test_result, self.unit);
                log::info!("Speed test completed successfully");
                Ok(())
            }
            Err(e) => {
                print_error(&e);
                Err(e)
            }
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Run both download and upload speed tests")]
    Start,

    #[command(about = "Run download speed test only")]
    Download,

    #[command(about = "Run upload speed test only")]
    Upload,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SpeedUnit {
    #[value(name = "bps")]
    Bps,
    #[value(name = "kbps")]
    Kbps,
    #[value(name = "mbps")]
    Mbps,
    #[value(name = "gbps")]
    Gbps,
}

impl SpeedUnit {
    pub fn convert(&self, bytes_per_second: f64) -> f64 {
        let bits_per_second = bytes_per_second * 8.0;
        match self {
            SpeedUnit::Bps => bits_per_second,
            SpeedUnit::Kbps => bits_per_second / 1_000.0,
            SpeedUnit::Mbps => bits_per_second / 1_000_000.0,
            SpeedUnit::Gbps => bits_per_second / 1_000_000_000.0,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SpeedUnit::Bps => "bps",
            SpeedUnit::Kbps => "Kbps",
            SpeedUnit::Mbps => "Mbps",
            SpeedUnit::Gbps => "Gbps",
        }
    }
}

async fn run_full_test(speed_test: &SpeedTest, unit: SpeedUnit) -> Result<SpeedTestResult> {
    // Download test with progress
    let download_pb = create_progress_bar("Testing download speed", unit);
    let download_callback =
        create_progress_callback(download_pb.clone(), "Testing download speed", unit);
    let download = speed_test
        .test_download_with_progress(Some(download_callback))
        .await?;
    download_pb.finish_with_message("✔ Testing download speed".to_string());

    // Upload test with progress
    let upload_pb = create_progress_bar("Testing upload speed", unit);
    let upload_callback = create_progress_callback(upload_pb.clone(), "Testing upload speed", unit);
    let upload = speed_test
        .test_upload_with_progress(Some(upload_callback))
        .await?;
    upload_pb.finish_with_message("✔ Testing upload speed".to_string());

    Ok(SpeedTestResult::new(Some(download), Some(upload)))
}

async fn run_download_test(speed_test: &SpeedTest, unit: SpeedUnit) -> Result<SpeedTestResult> {
    let pb = create_progress_bar("Testing download speed", unit);
    let callback = create_progress_callback(pb.clone(), "Testing download speed", unit);
    let download = speed_test
        .test_download_with_progress(Some(callback))
        .await?;
    pb.finish_with_message("✔ Testing download speed".to_string());

    Ok(SpeedTestResult::new(Some(download), None))
}

async fn run_upload_test(speed_test: &SpeedTest, unit: SpeedUnit) -> Result<SpeedTestResult> {
    let pb = create_progress_bar("Testing upload speed", unit);
    let callback = create_progress_callback(pb.clone(), "Testing upload speed", unit);
    let upload = speed_test.test_upload_with_progress(Some(callback)).await?;
    pb.finish_with_message("✔ Testing upload speed".to_string());

    Ok(SpeedTestResult::new(None, Some(upload)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_unit_convert_bps() {
        let unit = SpeedUnit::Bps;
        let bytes_per_second = 1000.0;
        let result = unit.convert(bytes_per_second);
        assert_eq!(result, 8000.0); // 1000 bytes/s = 8000 bits/s
    }

    #[test]
    fn test_speed_unit_convert_kbps() {
        let unit = SpeedUnit::Kbps;
        let bytes_per_second = 1000.0;
        let result = unit.convert(bytes_per_second);
        assert_eq!(result, 8.0); // 1000 bytes/s = 8 Kbps
    }

    #[test]
    fn test_speed_unit_convert_mbps() {
        let unit = SpeedUnit::Mbps;
        let bytes_per_second = 1_000_000.0;
        let result = unit.convert(bytes_per_second);
        assert_eq!(result, 8.0); // 1,000,000 bytes/s = 8 Mbps
    }

    #[test]
    fn test_speed_unit_convert_gbps() {
        let unit = SpeedUnit::Gbps;
        let bytes_per_second = 1_000_000_000.0;
        let result = unit.convert(bytes_per_second);
        assert_eq!(result, 8.0); // 1,000,000,000 bytes/s = 8 Gbps
    }

    #[test]
    fn test_speed_unit_as_str() {
        assert_eq!(SpeedUnit::Bps.as_str(), "bps");
        assert_eq!(SpeedUnit::Kbps.as_str(), "Kbps");
        assert_eq!(SpeedUnit::Mbps.as_str(), "Mbps");
        assert_eq!(SpeedUnit::Gbps.as_str(), "Gbps");
    }
}
