use std::sync::Arc;

use reqwest::Client;

use crate::{
    ErrorResponse,
    errors::{self, AppError, AppResult},
    stripe::v1::charges::charges::ChargesResponse,
};

/// DOCS Reference: https://docs.stripe.com/api/
#[allow(non_camel_case_types)]
pub struct v1 {
    key: Arc<String>,
    client: Client,
}

impl v1 {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self { key, client }
    }
    /// /v1/charges:
    ///
    /// List of history of charges created will listed in charges endpoint
    /// this function also will return ChargesResponse struct
    pub async fn charges(&self) -> AppResult<ChargesResponse> {
        let response = self
            .client
            .get("https://api.stripe.com/v1/charges")
            .basic_auth(&*self.key, Some(""))
            .send()
            .await
            .map_err(errors::AppError::Network)?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(errors::AppError::Api(body));
        }

        let data = response
            .json::<ChargesResponse>()
            .await
            .map_err(errors::AppError::Network)?;

        Ok(data)
    }
}
