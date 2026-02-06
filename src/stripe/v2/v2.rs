/// DOCS Reference: https://docs.stripe.com/api/v2/core/accounts
use std::sync::Arc;

use reqwest::Client;

use crate::stripe::v2::core::Core;

#[allow(non_camel_case_types)]
pub struct v2 {
    key: Arc<String>,
    client: Client,
}

impl v2 {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self { key, client }
    }

    pub fn core(self) -> Core {
        Core::new(self.key, self.client)
    }
}
