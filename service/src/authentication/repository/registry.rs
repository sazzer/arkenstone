use super::Provider;
use crate::authentication::ProviderName;
#[cfg(test)]
use faux;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of the providers that we can work with
#[cfg_attr(not(test), derive(Clone))]
#[cfg_attr(test, faux::create)]
pub struct Registry {
  providers: HashMap<ProviderName, Arc<dyn Provider>>,
}

#[cfg_attr(test, faux::methods)]
impl Registry {
  /// Build a new registry
  pub fn new(providers: HashMap<ProviderName, Arc<dyn Provider>>) -> Self {
    Self { providers }
  }

  /// Get the list of providers from the registry
  pub fn list_providers(&self) -> Vec<&ProviderName> {
    self.providers.iter().map(|(key, _)| key).collect()
  }

  /// Attempt to get the requested provider from the registry
  pub fn find_provider(&self, provider: &ProviderName) -> Option<&Arc<dyn Provider>> {
    self.providers.get(provider)
  }
}

/// Builder to use to build the provider registry
#[derive(Default)]
pub struct RegistryBuilder {
  providers: HashMap<ProviderName, Arc<dyn Provider>>,
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Clone for Registry {
  fn clone(&self) -> Self {
    Self {
      providers: self.providers.clone(),
    }
  }
}

impl RegistryBuilder {
  /// Add a new Provider to the builder
  pub fn with_provider<S>(&mut self, name: S, provider: Arc<dyn Provider>) -> &mut Self
  where
    S: Into<String>,
  {
    let provider_name: ProviderName = name.into().parse().unwrap();
    self.providers.insert(provider_name, provider.clone());

    self
  }

  /// Actually build the registry from what we've so far registered
  pub fn build(&self) -> Registry {
    Registry::new(self.providers.clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::authentication::repository::MockProvider;
  use galvanic_assert::{
    assert_that,
    matchers::{collection::*, variant::*, *},
  };

  #[test]
  fn test_list_providers() {
    let registry = RegistryBuilder::default()
      .with_provider("twitter", Arc::new(MockProvider::faux()))
      .with_provider("google", Arc::new(MockProvider::faux()))
      .build();

    let providers = registry.list_providers();

    let google: ProviderName = "google".parse().unwrap();
    let twitter: ProviderName = "twitter".parse().unwrap();

    assert_that!(&providers, contains_in_any_order(vec![&google, &twitter]));
  }

  #[test]
  fn test_get_unknown_provider() {
    let registry = RegistryBuilder::default()
      .with_provider("twitter", Arc::new(MockProvider::faux()))
      .with_provider("google", Arc::new(MockProvider::faux()))
      .build();

    let result = registry.find_provider(&"facebook".parse().unwrap());
    assert_that!(&result.is_none(), eq(true));
  }

  #[test]
  fn test_get_known_provider() {
    let registry = RegistryBuilder::default()
      .with_provider("twitter", Arc::new(MockProvider::faux()))
      .with_provider("google", Arc::new(MockProvider::faux()))
      .build();

    let result = registry.find_provider(&"twitter".parse().unwrap());
    assert_that!(&result, maybe_some(any_value())); // TODO: Assert it's the correct provider
  }
}
