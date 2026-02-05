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

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    let client = Stripe::new(env::var("STRIPE_API_KEY").expect("STRIPE_API_KEY is not set"));

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
