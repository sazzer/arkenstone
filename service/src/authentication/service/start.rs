use super::AuthenticationService;
#[cfg(test)]
use super::_FauxOriginal_AuthenticationService;
use crate::authentication::{ProviderName, StartAuth, StartAuthDetails, StartError};
#[cfg(test)]
use faux;
use uuid::Uuid;

#[cfg_attr(test, faux::methods)]
impl StartAuth for AuthenticationService {
  fn start_auth(&self, provider: &ProviderName) -> Result<StartAuthDetails, StartError> {
    let provider = self
      .registry
      .find_provider(provider)
      .ok_or(StartError::UnknownProvider)?;

    let nonce = Uuid::new_v4().to_string();
    let url = provider.start_auth(&nonce);
    Ok(StartAuthDetails {
      url: url,
      nonce: nonce,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::authentication::repository::{MockProvider, Provider, Registry};
  use galvanic_assert::{
    assert_that, is_variant,
    matchers::{variant::*, *},
  };
  use std::str::FromStr;
  use std::sync::Arc;

  #[test]
  fn test_start_unknown_provider() {
    let mut registry = Registry::faux();
    let provider_name = ProviderName::from_str("google").unwrap();

    unsafe {
      faux::when!(registry.find_provider).then(|input_provider_name| {
        assert_that!(&input_provider_name, eq(&provider_name));
        None
      });
    }

    let sut = AuthenticationService::new(registry);

    let result = sut.start_auth(&provider_name);
    assert_that!(&result, maybe_err(eq(StartError::UnknownProvider)));
  }

  #[test]
  fn test_start_known_provider() {
    let provider_name = ProviderName::from_str("google").unwrap();

    let mut provider = MockProvider::faux();
    let mut captured_nonce: Option<String> = None;
    unsafe {
      faux::when!(provider.start_auth).then(|nonce| {
        captured_nonce = Some(nonce.clone());
        "https://www.google.com".to_owned()
      });
    }

    let mut registry = Registry::faux();
    let provider_arc: Arc<dyn Provider> = Arc::new(provider);
    unsafe {
      faux::when!(registry.find_provider).then(|input_provider_name| {
        assert_that!(&input_provider_name, eq(&provider_name));
        Some(&provider_arc)
      });
    }

    let sut = AuthenticationService::new(registry);

    let result = sut.start_auth(&provider_name);

    assert_that!(&captured_nonce, is_variant!(Some));
    assert_that!(&result, is_variant!(Ok));

    let result = result.unwrap();
    assert_that!(&result.nonce, eq(captured_nonce.unwrap()));
    assert_that!(&result.url, eq("https://www.google.com".to_owned()));
  }
}
