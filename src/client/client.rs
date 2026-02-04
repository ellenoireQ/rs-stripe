pub struct CreateClient {
    client: &'static str,
}

impl CreateClient {
    /// Storing client_value to client
    pub fn new(client_value: &'static str) -> Self {
        Self {
            client: client_value,
        }
    }

    /// Returning client value
    pub fn show(self) -> &'static str {
        return self.client;
    }
}
