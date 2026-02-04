use std::io::{self, Error};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Message {
    message: String,
    r#type: String,
}
#[derive(Deserialize, Debug)]
struct Errors {
    error: Message,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://api.stripe.com")
        .await?
        .json::<Errors>()
        .await?;

    println!("body: {:?}", body.error.r#type);
    Ok(())
}
