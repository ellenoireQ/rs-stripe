use crate::stripe::v1::v1::v1;
pub struct Stripe {
    key: &'static str,
}

impl Stripe {
    /// Storing key (lifetime)
    pub fn new(key: &'static str) -> Self {
        Self { key: key }
    }

    /// Returning key value
    pub fn get_key(&self) -> &'static str {
        return self.key;
    }

    /// Using v1 as fallback to better type structure
    /// for example: stripe.v1().X();
    pub fn v1(&self) -> v1 {
        v1::new()
    }
}
