use super::model::ProviderRedirect;
use crate::authentication::{AuthenticationService, ProviderName, StartAuth};
use actix_web::web;

/// Actix handler to start authentication with a provider
pub async fn start(
  authentication_service: web::Data<AuthenticationService>,
  path: web::Path<(ProviderName,)>,
) -> ProviderRedirect {
  let response = authentication_service.start_auth(&path.0);

  ProviderRedirect {
    url: response.url,
    provider: path.0.clone(),
    nonce: response.nonce,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::authentication::{ProviderName, StartAuthDetails};
  use faux;
  use galvanic_assert::{assert_that, matchers::*};
  use std::str::FromStr;

  #[actix_rt::test]
  async fn test_start_authentication() {
    let mut service = AuthenticationService::faux();
    unsafe {
      faux::when!(service.start_auth).then(|input_provider| {
        assert_that!(
          &input_provider,
          eq(ProviderName::from_str("google").unwrap())
        );

        StartAuthDetails {
          url: "http://www.google.com".to_owned(),
          nonce: "someNonce".to_owned(),
        }
      });
    }

    let result = start(
      web::Data::new(service),
      web::Path::from((ProviderName::from_str("google").unwrap(),)),
    )
    .await;

    assert_that!(&result.url, eq("http://www.google.com".to_owned()));
    assert_that!(&result.nonce, eq("someNonce".to_owned()));
    assert_that!(
      &result.provider,
      eq(ProviderName::from_str("google").unwrap())
    );
  }
}
