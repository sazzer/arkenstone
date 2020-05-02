use super::AuthenticationService;
use crate::authentication::{ProviderName, StartAuth, StartAuthDetails, StartError};
use uuid::Uuid;

impl StartAuth for AuthenticationService {
    fn start_auth(&self, provider: &ProviderName) -> Result<StartAuthDetails, StartError> {
        let provider = self
            .registry
            .find_provider(provider)
            .ok_or(StartError::UnknownProvider)?;

        let nonce = Uuid::new_v4().to_string();
        let url = provider.start_auth(&nonce);
        Ok(StartAuthDetails { url, nonce })
    }
}
