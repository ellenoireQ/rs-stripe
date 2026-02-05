use serde::de::DeserializeOwned;

use crate::errors::AppResult;

pub struct Charges {
    pub query: Vec<(&'static str, String)>,
}

impl Charges {
    pub fn new() -> Self {
        Self { query: Vec::new() }
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query.push(("expand[]", value.into()));
        self
    }

    pub async fn get<T>(self) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        Ok(todo!())
    }
}
