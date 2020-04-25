use crate::authentication::ProviderName;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CompleteError {
  #[error("The requested authentication provider is unknown")]
  UnknownProvider,
}

/// Base trait for completing authentication with a provider
pub trait CompleteAuth {
  /// Complete authentication with the provider and return the external user details
  fn complete_auth(
    &self,
    provider: &ProviderName,
    params: HashMap<String, String>,
  ) -> Result<(), CompleteError>;
}
