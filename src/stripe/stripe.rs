pub struct Stripe {
    key: &'static str,
}

impl Stripe {
    /// Storing key (lifetime)
    pub fn new(key: &'static str) -> Self {
        Self { key: key }
    }

    /// Returning key value
    pub fn get_key(self) -> &'static str {
        return self.key;
    }
}
