use super::{ProviderName, ProviderPlayerId};

#[derive(Debug, Clone, PartialEq)]
pub struct Login {
    pub provider: ProviderName,
    pub provider_id: ProviderPlayerId,
    pub display_name: String,
}
