use crate::{
    errors::{AppError, AppResult},
    stripe::v2::accounts::account_request::CreateAccountRequest,
};
use reqwest::Client;
use std::sync::Arc;

pub struct Core {
    key: Arc<String>,
    client: Client,
}

pub struct Configure {
    key: Arc<String>,
    client: Client,
}

pub struct Accounts {
    key: Arc<String>,
    client: Client,
}

impl Core {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self { key, client }
    }

    pub fn create(self) -> Configure {
        Configure {
            key: self.key,
            client: self.client,
        }
    }

    pub fn close(self) {
        todo!()
    }
}

impl Configure {
    pub async fn accounts(self, payload: CreateAccountRequest) -> Result<(), AppError> {
        let executor = Accounts {
            key: self.key,
            client: self.client,
        };

        executor.send_request(payload).await
    }
}

/// /v2/core/accounts
///
/// Stripe Docs:
/// https://docs.stripe.com/api/accounts
///
/// An Account represents a company, individual, or other entity that
/// interacts with Stripe. It stores identifying information and
/// configuration for enabled features.
impl Accounts {
    pub async fn send_request(self, payload: CreateAccountRequest) -> Result<(), AppError> {
        let response = self
            .client
            .post("https://api.stripe.com/v2/core/accounts")
            .bearer_auth(self.key)
            .header("Stripe-Version", "2026-01-28.preview")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Api(e.to_string()))?;

        if response.status().is_success() {
            let body = response
                .text()
                .await
                .map_err(|e| AppError::Api(e.to_string()))?;
            println!("Success: {}", body);
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(AppError::Api(format!("Error {}: {}", status, body)))
        }
    }
}
