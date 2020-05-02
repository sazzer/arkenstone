use crate::players::ProviderName;

/// Use Case to get a list of authentication providers
pub trait ListProviders {
    /// Get a list of all known authentication providers
    fn list_providers(&self) -> Vec<&ProviderName>;
}
