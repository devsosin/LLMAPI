use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Unauthorized Error")]
    UnauthorizedError,

    #[error("RateLimited Error")]
    RateLimitedError,

    #[error("Internal error: {0}")]
    InternalError(String),
}
