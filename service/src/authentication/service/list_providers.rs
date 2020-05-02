use super::AuthenticationService;
use crate::authentication::{ListProviders, ProviderName};

impl ListProviders for AuthenticationService {
    /// Get a list of all known authentication providers
    fn list_providers(&self) -> Vec<&ProviderName> {
        self.registry.list_providers()
    }
}
