use crate::authentication::ProviderName;

/// Details describing how to start authenticating with a provider
pub struct StartAuthDetails {
  pub url: String,
  pub nonce: String,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum StartError {
  #[error("The reqeusted authentication provider is unknown")]
  UnknownProvider,
}

/// Base trait for starting authentication with a provider
pub trait StartAuth {
  /// Build a URL to redirect the user to in order to start authentication
  fn start_auth(&self, provider: &ProviderName) -> Result<StartAuthDetails, StartError>;
}
