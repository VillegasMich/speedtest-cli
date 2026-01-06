use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpeedTestError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Test failed: {0}")]
    TestFailed(String),

    #[allow(dead_code)]
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[allow(dead_code)]
    #[error("Timeout occurred during speed test")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, SpeedTestError>;
