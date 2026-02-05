use std::sync::Arc;

use crate::stripe::v1::v1::v1;
pub struct Stripe {
    key: Arc<String>,
}

impl Stripe {
    /// Storing key
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: Arc::new(key.into()),
        }
    }

    /// Returning key value
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Using v1 as fallback to better type structure
    /// for example: stripe.v1().X();
    pub fn v1(&self) -> v1 {
        v1::new(self.key.clone())
    }
}
