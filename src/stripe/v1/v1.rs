use std::sync::Arc;

use reqwest::Client;

use crate::{
    errors::{self, AppError, AppResult},
    stripe::v1::charges::{charges::Charges, definition::ChargesResponse},
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
    pub fn charges(&self) -> Charges {
        Charges::new(self.key.clone(), self.client.clone())
    }
}
