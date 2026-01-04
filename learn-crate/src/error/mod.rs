use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::Error),
}
