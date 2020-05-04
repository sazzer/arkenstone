use super::AuthenticationService;
use crate::authentication::{StartAuth, StartAuthDetails, StartError};
use crate::players::ProviderName;
use uuid::Uuid;

impl StartAuth for AuthenticationService {
    fn start_auth(&self, provider_name: &ProviderName) -> Result<StartAuthDetails, StartError> {
        let provider = self
            .registry
            .find_provider(provider_name)
            .ok_or(StartError::UnknownProvider)?;

        let nonce = Uuid::new_v4().to_string();

        let span = tracing::error_span!("start_auth", provider=?provider_name, nonce=?nonce);
        let _enter = span.enter();

        let url = provider.start_auth(&nonce);

        log::info!("Built URL: {}", url);

        Ok(StartAuthDetails { url, nonce })
    }
}
