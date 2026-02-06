use std::sync::Arc;

use reqwest::Client;

use crate::stripe::{v1::v1::v1, v2::v2::v2};
pub struct Stripe {
    key: Arc<String>,
    client: Client,
}

impl Stripe {
    /// Storing key
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: Arc::new(key.into()),
            client: Client::new(),
        }
    }

    /// Returning key value
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Using v1 as fallback to better type structure
    /// for example: stripe.v1().X();
    pub fn v1(&self) -> v1 {
        v1::new(self.key.clone(), self.client.clone())
    }
    /// Using v2 as fallback to better type structure
    /// for example: stripe.v1().X();
    pub fn v2(&self) -> v2 {
        v2::new(self.key.clone(), self.client.clone())
    }
}
