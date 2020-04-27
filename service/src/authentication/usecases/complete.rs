use crate::authentication::ProviderName;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CompleteError {
  #[error("The requested authentication provider is unknown")]
  UnknownProvider,
}

/// Base trait for completing authentication with a provider
#[async_trait]
pub trait CompleteAuth {
  /// Complete authentication with the provider and return the external user details
  async fn complete_auth(
    &self,
    provider: &ProviderName,
    params: HashMap<String, String>,
  ) -> Result<(), CompleteError>;
}
