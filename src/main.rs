use core::fmt;

use crate::errors::{AppError, AppResult};
use serde::Deserialize;
mod errors;

#[derive(Deserialize, Debug)]
pub struct ErrorBody {
    pub message: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.error.message, self.error.r#type)
    }
}

async fn fetch() -> AppResult<()> {
    let res = reqwest::get("https://api.stripe.com").await?;
    let status = res.status();
    let body = res.text().await?;

    if !status.is_success() {
        let err: ErrorResponse = serde_json::from_str(&body)?;
        return Err(errors::AppError::Api(err));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    match fetch().await {
        Err(AppError::Api(e)) => {
            println!("stripe error type: {}", e.error.r#type);
            println!("message: {}", e.error.message);
        }
        Err(e) => {
            println!("other error: {}", e);
        }
        Ok(_) => {
            println!("success");
        }
    }

    Ok(())
}

