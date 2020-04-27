use async_trait::async_trait;
use std::collections::HashMap;

/// Base trait for starting authentication with a provider
pub trait ProviderStartAuth {
  /// Build a URL to redirect the user to in order to start authentication
  fn start_auth(&self, nonce: &str) -> String;
}

/// Base trait for completing authentication with a provider
#[async_trait]
pub trait ProviderCompleteAuth {
  /// Complete authentication and return the details of the user that has authenticated
  async fn complete_auth(&self, params: HashMap<String, String>) -> Result<(), ()>;
}

/// Trait that all Providers will implement
pub trait Provider: ProviderStartAuth + ProviderCompleteAuth + Sync + Send {}

#[cfg(test)]
#[cfg_attr(test, faux::create)]
pub struct MockProvider {}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl ProviderStartAuth for MockProvider {
  fn start_auth(&self, _nonce: &str) -> String {
    todo!()
  }
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
#[async_trait]
impl ProviderCompleteAuth for MockProvider {
  async fn complete_auth(&self, params: HashMap<String, String>) -> Result<(), ()> {
    todo!()
  }
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Provider for MockProvider {}
