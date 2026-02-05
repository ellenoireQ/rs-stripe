/// Endpoint https://api.stripe.com/v1/charges
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargesResponse {
    pub object: String,
    pub data: Vec<Charge>,
    pub has_more: bool,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Charge {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub amount_captured: i64,
    pub amount_refunded: i64,

    pub application: Option<String>,
    pub application_fee: Option<String>,
    pub application_fee_amount: Option<i64>,

    pub balance_transaction: String,
    pub billing_details: BillingDetails,

    pub calculated_statement_descriptor: Option<String>,
    pub captured: bool,
    pub created: i64,
    pub currency: String,
    pub customer: Option<String>,

    pub description: Option<String>,
    pub destination: Option<String>,
    pub dispute: Option<String>,
    pub disputed: bool,

    pub failure_balance_transaction: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,

    pub fraud_details: HashMap<String, String>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,

    pub on_behalf_of: Option<String>,
    pub order: Option<String>,

    pub outcome: Option<Outcome>,

    pub paid: bool,
    pub payment_intent: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_details: Option<PaymentMethodDetails>,

    pub radar_options: HashMap<String, String>,

    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub receipt_url: Option<String>,

    pub refunded: bool,
    pub review: Option<String>,
    pub shipping: Option<String>,
    pub source: Option<String>,
    pub source_transfer: Option<String>,

    pub statement_descriptor: Option<String>,
    pub statement_descriptor_suffix: Option<String>,

    pub status: String,
    pub transfer_data: Option<String>,
    pub transfer_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingDetails {
    pub address: Address,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub advice_code: Option<String>,
    pub network_advice_code: Option<String>,
    pub network_decline_code: Option<String>,
    pub network_status: Option<String>,
    pub reason: Option<String>,
    pub risk_level: Option<String>,
    pub risk_score: Option<i64>,
    pub seller_message: Option<String>,
    #[serde(rename = "type")]
    pub outcome_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    pub card: Option<Card>,
    #[serde(rename = "type")]
    pub method_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub amount_authorized: Option<i64>,
    pub authorization_code: Option<String>,
    pub brand: Option<String>,
    pub checks: Option<CardChecks>,
    pub country: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,

    pub extended_authorization: Option<StatusObject>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub incremental_authorization: Option<StatusObject>,
    pub installments: Option<String>,
    pub last4: Option<String>,
    pub mandate: Option<String>,
    pub multicapture: Option<StatusObject>,
    pub network: Option<String>,
    pub network_token: Option<NetworkToken>,
    pub network_transaction_id: Option<String>,
    pub overcapture: Option<Overcapture>,
    pub regulated_status: Option<String>,
    pub three_d_secure: Option<String>,
    pub wallet: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardChecks {
    pub address_line1_check: Option<String>,
    pub address_postal_code_check: Option<String>,
    pub cvc_check: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusObject {
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkToken {
    pub used: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Overcapture {
    pub maximum_amount_capturable: Option<i64>,
    pub status: Option<String>,
}
