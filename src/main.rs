use core::fmt;
use std::env;

use crate::{
    errors::{AppError, AppResult},
    stripe::{stripe::Stripe, v2::accounts::account_request::*},
};
use dotenvy::dotenv;
use serde::Deserialize;
use serde_json::Value;
mod errors;
mod stripe;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    let client = Stripe::new(env::var("STRIPE_API_KEY").expect("STRIPE_API_KEY is not set"));

    //println!("{:?}", client.get_key());
    //let m = client.v1().charges().query("value").get::<Value>().await?;
    let payload = CreateAccountRequest {
        contact_email: "furever@example.com".to_string(),
        display_name: "Furever".to_string(),
        identity: Identity {
            country: "us".to_string(),
            entity_type: "company".to_string(),
            business_details: BusinessDetails {
                registered_name: "Furever".to_string(),
            },
        },
        configuration: Configuration {
            customer: CustomerConfig {
                capabilities: CustomerCapabilities {
                    automatic_indirect_tax: FeatureRequest { requested: true },
                },
            },
            merchant: MerchantConfig {
                capabilities: MerchantCapabilities {
                    card_payments: FeatureRequest { requested: true },
                },
            },
        },
        defaults: Defaults {
            responsibilities: Responsibilities {
                fees_collector: "stripe".to_string(),
                losses_collector: "stripe".to_string(),
            },
        },
        dashboard: "full".to_string(),
        include: vec![
            "configuration.merchant".to_string(),
            "configuration.customer".to_string(),
            "identity".to_string(),
            "defaults".to_string(),
        ],
    };
    //let m = client.v2().core().create().accounts(payload).await?;

    let m = client.v1().payment_intents().list().await?;

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
