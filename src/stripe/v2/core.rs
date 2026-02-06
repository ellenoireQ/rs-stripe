use std::sync::Arc;

use reqwest::Client;

#[allow(non_camel_case_types)]
pub struct core {
    key: Arc<String>,
    client: Client,
    create: bool,
}

impl core {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self {
            key,
            client,
            create: false,
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
    pub fn accounts(self) {
        if self.create {
            todo!()
        }
    }
    pub fn close(self) {
        todo!()
    }
}
