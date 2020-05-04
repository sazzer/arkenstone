use crate::authentication::AuthenticationService;
use crate::players::ProviderName;
use uuid::Uuid;

/// Details describing how to start authenticating with a provider
pub struct StartAuthDetails {
    pub url: String,
    pub nonce: String,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum StartError {
    #[error("The requested authentication provider is unknown")]
    UnknownProvider,
}

impl AuthenticationService {
    pub fn start_auth(&self, provider_name: &ProviderName) -> Result<StartAuthDetails, StartError> {
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
