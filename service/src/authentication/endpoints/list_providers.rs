use crate::authentication::{AuthenticationService, ListProviders, ProviderName};
use actix_web::web;
use serde::Serialize;

/// API Model to represent the list of providers
#[derive(Serialize)]
pub struct ProvidersList {
  pub providers: Vec<ProviderName>,
}

/// Actix handler to list the authentication providers
pub async fn list_providers(
  authentication_service: web::Data<AuthenticationService>,
) -> web::Json<ProvidersList> {
  let mut providers = authentication_service.list_providers();
  providers.sort();

  web::Json(ProvidersList {
    providers: providers.into_iter().map(|p| p.clone()).collect(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use faux;
  use galvanic_assert::{assert_that, matchers::collection::*};

  #[actix_rt::test]
  async fn test_list_providers() {
    let google: ProviderName = "google".parse().unwrap();
    let twitter: ProviderName = "twitter".parse().unwrap();

    let mut service = AuthenticationService::faux();
    unsafe {
      faux::when!(service.list_providers).then(|_| vec![&twitter, &google]);
    }

    let result = list_providers(web::Data::new(service)).await;

    assert_that!(&result.providers, contains_in_order(vec![google, twitter]));
  }
}
