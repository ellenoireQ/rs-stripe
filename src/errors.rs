use std::fmt;

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid request")]
    InvalidRequest,

    #[error("api error: {0}")]
    Api(String),

    #[error("network error")]
    Network(#[from] reqwest::Error),

    #[error("json parse error")]
    Parse(#[from] serde_json::Error),
}

pub type AppResult<T> = Result<T, AppError>;
