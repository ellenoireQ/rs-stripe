use std::sync::Arc;

use reqwest::Client;
use serde_json::Value;

use crate::errors::AppError;

pub struct PaymentIntent {
    key: Arc<String>,
    client: Client,
}
impl PaymentIntent {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self { key, client }
    }

    pub async fn create(self, amount: u32, currency: String) -> Result<(), AppError> {
        let res = self
            .client
            .post("https://api.stripe.com/v1/payment_intents")
            .bearer_auth(self.key)
            .header("Stripe-Version", "2026-01-28.preview")
            .form(&[("amount", amount.to_string()), ("currency", currency)])
            .send()
            .await
            .map_err(|e| AppError::Api(e.to_string()))?;

        Ok(match res {
            status if status.status().is_success() => {}
            status => {
                return Err(AppError::Api(format!(
                    "API request failed with status: {}",
                    status.status()
                )));
            }
        })
    }

    pub async fn list(self) -> Result<Value, AppError> {
        let res = self
            .client
            .get("https://api.stripe.com/v1/payment_intents")
            .bearer_auth(self.key)
            .header("Stripe-Version", "2026-01-28.preview")
            .send()
            .await
            .map_err(|e| AppError::Api(e.to_string()))?;

        Ok(match res {
            status if status.status().is_success() => status
                .json::<Value>()
                .await
                .map_err(|e| AppError::Api(e.to_string()))?,
            status => {
                return Err(AppError::Api(format!(
                    "API request failed with status: {}",
                    status.status()
                )));
            }
        })
    }
}

pub struct Amount {
    amount: u32,
}

impl Amount {
    pub fn new() -> Self {
        Self { amount: 0 }
    }

    pub fn amount(self, amount: u32) -> Self {
        Self { amount }
    }
}
