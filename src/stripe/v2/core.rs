use std::sync::Arc;

use reqwest::Client;

use crate::{
    errors::{AppError, AppResult},
    stripe::v2::accounts::account_request::CreateAccountRequest,
};

#[allow(non_camel_case_types)]
pub struct core {
    key: Arc<String>,
    client: Client,
    create: bool,
    payload: Option<CreateAccountRequest>,
}

impl core {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self {
            key,
            client,
            create: false,
            payload: None,
        }
    }

    /// Create mode
    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }

    /// /v2/core/accounts
    ///
    /// Stripe Docs:
    /// https://docs.stripe.com/api/accounts
    ///
    /// An Account represents a company, individual, or other entity that
    /// interacts with Stripe. It stores identifying information and
    /// configuration for enabled features.
    pub async fn accounts(self) -> Result<(), AppError> {
        if self.create {
            let response = self
                .client
                .post("https://api.stripe.com/v2/core/accounts")
                .bearer_auth(self.key)
                .header("Stripe-Version", "2026-01-28.preview")
                .json(&self.payload)
                .send()
                .await
                .map_err(|e| AppError::Api(e.to_string()));

            match response {
                Ok(res) => {
                    let status = res.status();
                    let body = res.text().await.map_err(|e| AppError::Api(e.to_string()))?;
                    if status.is_success() {
                        println!("Success: {}", body);
                    } else {
                        println!("Error {}: {}", status, body);
                    }
                }
                Err(e) => return Err(AppError::Api(e.to_string())),
            }
        } else {
            return Err(AppError::InvalidRequest);
        }
        Ok(())
    }
    pub fn configure(self, payload: CreateAccountRequest) -> Self {
        let payload = payload;
        Self {
            payload: Some(payload),
            ..self
        }
    }

    pub fn close(self) {
        todo!()
    }
}
