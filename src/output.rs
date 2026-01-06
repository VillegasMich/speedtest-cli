use crate::cli::SpeedUnit;
use crate::error::SpeedTestError;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

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

pub fn create_progress_bar(message: &str, _unit: SpeedUnit) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .expect("Failed to set progress bar template"),
    );
    pb.set_message(format!("{} - Initializing...", message));
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

pub fn create_progress_callback(
    pb: ProgressBar,
    message: &str,
    unit: SpeedUnit,
) -> Box<dyn Fn(f64) + Send + Sync> {
    let message = message.to_string();
    Box::new(move |speed: f64| {
        let converted = unit.convert(speed);
        pb.set_message(format!("{} - {:.2} {}", message, converted, unit.as_str()));
    })
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
