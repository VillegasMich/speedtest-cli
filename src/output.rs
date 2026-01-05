use crate::cli::SpeedUnit;
use crate::error::SpeedTestError;
use colored::*;

pub struct SpeedTestResult {
    pub download_speed: Option<f64>,
    pub upload_speed: Option<f64>,
}

impl SpeedTestResult {
    pub fn new(download_speed: Option<f64>, upload_speed: Option<f64>) -> Self {
        Self {
            download_speed,
            upload_speed,
        }
    }
}

pub fn print_success(result: SpeedTestResult, unit: SpeedUnit) {
    println!("\n{}", "Speed Test Results".bold().green());
    println!("{}", "===================".green());

    if let Some(download) = result.download_speed {
        let converted = unit.convert(download);
        println!(
            "{} {:.2} {}",
            "Download Speed:".bold(),
            converted,
            unit.as_str().cyan()
        );
    }

    if let Some(upload) = result.upload_speed {
        let converted = unit.convert(upload);
        println!(
            "{} {:.2} {}",
            "Upload Speed:  ".bold(),
            converted,
            unit.as_str().cyan()
        );
    }

    println!();
}

pub fn print_error(error: &SpeedTestError) {
    eprintln!("\n{} {}", "Error:".bold().red(), error);
    log::error!("{}", error);
}

pub fn print_progress(message: &str) {
    println!("{} {}", "=>".bold().blue(), message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_test_result_new() {
        let result = SpeedTestResult::new(Some(1000.0), Some(500.0));
        assert_eq!(result.download_speed, Some(1000.0));
        assert_eq!(result.upload_speed, Some(500.0));
    }

    #[test]
    fn test_speed_test_result_download_only() {
        let result = SpeedTestResult::new(Some(1000.0), None);
        assert_eq!(result.download_speed, Some(1000.0));
        assert_eq!(result.upload_speed, None);
    }

    #[test]
    fn test_speed_test_result_upload_only() {
        let result = SpeedTestResult::new(None, Some(500.0));
        assert_eq!(result.download_speed, None);
        assert_eq!(result.upload_speed, Some(500.0));
    }
}
