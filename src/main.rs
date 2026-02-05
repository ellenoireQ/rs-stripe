use core::fmt;
use std::env;

use crate::{
    errors::{AppError, AppResult},
    stripe::stripe::Stripe,
};
use dotenvy::dotenv;
use serde::Deserialize;
mod errors;
mod stripe;

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
        return Err(errors::AppError::Api(err.to_string()));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    let client = Stripe::new(env::var("STRIPE_API_KEY").unwrap());

    println!("{:?}", client.get_key());
    let m = client.v1().charges().await?;

    println!("{:#?}", m);
    /*
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
    */
    Ok(())
}
