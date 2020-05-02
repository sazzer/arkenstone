use super::AuthenticationService;
use crate::authentication::ListProviders;
use crate::players::ProviderName;

impl ListProviders for AuthenticationService {
    /// Get a list of all known authentication providers
    fn list_providers(&self) -> Vec<&ProviderName> {
        self.registry.list_providers()
    }
}
