use crate::stripe::v1::charges::charges::ChargesResponse;

/// DOCS Reference: https://docs.stripe.com/api/
#[allow(non_camel_case_types)]
pub struct v1;

impl v1 {
    pub fn new() -> Self {
        Self
    }
    /// /v1/charges:
    ///
    /// List of history of charges created will listed in charges endpoint
    /// this function also will return ChargesResponse struct
    pub fn charges(&self) -> ChargesResponse {
        ChargesResponse {
            object: "list".to_string(),
            data: vec![],
            has_more: false,
            url: "https://api.stripe.com/v1/charges".to_string(),
        }
    }
}
