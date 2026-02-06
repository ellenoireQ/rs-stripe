use serde::Serialize;

#[derive(Serialize)]
pub struct CreateAccountRequest {
    pub contact_email: String,
    pub display_name: String,
    pub identity: Identity,
    pub configuration: Configuration,
    pub defaults: Defaults,
    pub dashboard: String,
    pub include: Vec<String>,
}

#[derive(Serialize)]
pub struct Identity {
    pub country: String,
    pub entity_type: String,
    pub business_details: BusinessDetails,
}

#[derive(Serialize)]
pub struct BusinessDetails {
    pub registered_name: String,
}

#[derive(Serialize)]
pub struct Configuration {
    pub customer: CustomerConfig,
    pub merchant: MerchantConfig,
}

#[derive(Serialize)]
pub struct CustomerConfig {
    pub capabilities: CustomerCapabilities,
}

#[derive(Serialize)]
pub struct CustomerCapabilities {
    pub automatic_indirect_tax: FeatureRequest,
}

#[derive(Serialize)]
pub struct MerchantConfig {
    pub capabilities: MerchantCapabilities,
}

#[derive(Serialize)]
pub struct MerchantCapabilities {
    pub card_payments: FeatureRequest,
}

#[derive(Serialize)]
pub struct FeatureRequest {
    pub requested: bool,
}

#[derive(Serialize)]
pub struct Defaults {
    pub responsibilities: Responsibilities,
}

#[derive(Serialize)]
pub struct Responsibilities {
    pub fees_collector: String,
    pub losses_collector: String,
}
