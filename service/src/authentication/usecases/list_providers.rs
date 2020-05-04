use crate::authentication::AuthenticationService;
use crate::players::ProviderName;

impl AuthenticationService {
    /// Get a list of all known authentication providers
    pub fn list_providers(&self) -> Vec<&ProviderName> {
        self.registry.list_providers()
    }
}
