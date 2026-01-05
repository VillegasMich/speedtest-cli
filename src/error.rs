use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpeedTestError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Test failed: {0}")]
    TestFailed(String),
}

pub type Result<T> = std::result::Result<T, SpeedTestError>;
