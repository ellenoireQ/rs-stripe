use std::sync::Arc;

use reqwest::Client;

#[allow(non_camel_case_types)]
pub struct core {
    key: Arc<String>,
    client: Client,
}

impl core {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self { key, client }
    }

    /// /v2/core/accounts
    ///
    /// Stripe Docs:
    /// https://docs.stripe.com/api/accounts
    ///
    /// An Account represents a company, individual, or other entity that
    /// interacts with Stripe. It stores identifying information and
    /// configuration for enabled features.
    pub fn accounts(self) {
        todo!()
    }
    pub fn close(self) {
        todo!()
    }
}
