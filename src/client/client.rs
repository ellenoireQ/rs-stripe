pub struct CreateClient {
    key: &'static str,
}

impl CreateClient {
    /// Storing key (lifetime)
    pub fn new(key: &'static str) -> Self {
        Self { key: key }
    }

    /// Returning key value
    pub fn get_key(self) -> &'static str {
        return self.key;
    }
}
