use std::sync::Arc;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::errors::{AppError, AppResult};

pub struct Charges {
    key: Arc<String>,
    client: Client,
    pub query: Vec<(&'static str, String)>,
    pub filter: String,
}

impl Charges {
    pub fn new(key: Arc<String>, client: Client) -> Self {
        Self {
            key,
            client,
            query: Vec::new(),
            filter: String::new(),
        }
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query.push(("expand[]", value.into()));
        self
    }

    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = value.into();
        self
    }

    pub async fn get<T>(self) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        if !self.query.is_empty() {
            if self.filter.is_empty() {
                return Err(AppError::InvalidRequest);
            }
            let res = self
                .client
                .get(format!("https://api.stripe.com/v1/charges/{}", self.filter))
                .bearer_auth(self.key.as_ref())
                .send()
                .await?
                .json::<T>()
                .await?;
            Ok(res)
        } else {
            let mut req = self
                .client
                .get("https://api.stripe.com/v1/charges")
                .bearer_auth(self.key.as_ref());
            for (k, v) in self.query {
                req = req.query(&[(k, v)]);
            }
            let res = req.send().await?.json::<T>().await?;
            Ok(res)
        }
    }
}
