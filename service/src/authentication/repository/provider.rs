#[cfg(test)]
use faux;

/// Base trait for starting authentication with a provider
pub trait ProviderStartAuth {
  /// Build a URL to redirect the user to in order to start authentication
  fn start_auth(&self, nonce: &String) -> String;
}

/// Base trait for completing authentication with a provider
pub trait ProviderCompleteAuth {}

/// Trait that all Providers will implement
pub trait Provider: ProviderStartAuth + ProviderCompleteAuth + Sync + Send {}

#[cfg(test)]
#[cfg_attr(test, faux::create)]
pub struct MockProvider {}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl ProviderStartAuth for MockProvider {
  fn start_auth(&self, _nonce: &String) -> String {
    todo!()
  }
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl ProviderCompleteAuth for MockProvider {}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Provider for MockProvider {}
