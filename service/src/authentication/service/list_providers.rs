use super::AuthenticationService;
use crate::authentication::{ListProviders, ProviderName};

impl ListProviders for AuthenticationService {
  /// Get a list of all known authentication providers
  fn list_providers(&self) -> Vec<&ProviderName> {
    self.registry.list_providers()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::authentication::repository::Registry;
  use galvanic_assert::{assert_that, matchers::collection::*};

  #[test]
  fn test_list_providers() {
    let google: ProviderName = "google".parse().unwrap();
    let twitter: ProviderName = "twitter".parse().unwrap();

    let mut registry = Registry::faux();
    unsafe {
      faux::when!(registry.list_providers).then(|_| vec![&google, &twitter]);
    }

    let sut = AuthenticationService::new(registry);

    let result = sut.list_providers();

    assert_that!(&result, contains_in_any_order(vec![&google, &twitter]));
  }
}
