use std::sync::Arc;

use reqwest::Client;

use crate::stripe::v1::charges::charges::ChargesResponse;

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
    pub async fn charges(&self) -> Result<ChargesResponse, reqwest::Error> {
        println!("Stripe API Key: {}", self.key);

        let res = self
            .client
            .get("https://api.stripe.com/v1/charges")
            .basic_auth(&*self.key, Some(""))
            .send()
            .await?
            .json::<ChargesResponse>()
            .await?;
        Ok(res)
    }
}
