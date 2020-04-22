use crate::authentication::ProviderName;

/// Details describing how to start authenticating with a provider
pub struct StartAuthDetails {
  pub url: String,
  pub nonce: String,
}

/// Base trait for starting authentication with a provider
pub trait StartAuth {
  /// Build a URL to redirect the user to in order to start authentication
  fn start_auth(&self, provider: ProviderName) -> StartAuthDetails;
}
