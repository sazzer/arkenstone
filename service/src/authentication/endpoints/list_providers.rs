use crate::authentication::{AuthenticationService, ListProviders, ProviderName};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

/// API Model to represent the list of providers
#[derive(Serialize)]
pub struct ProvidersList {
  pub providers: Vec<ProviderName>,
}

impl Responder for ProvidersList {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    ready(Ok(HttpResponse::Ok().json(self)))
  }
}

/// Actix handler to list the authentication providers
pub async fn list_providers(
  authentication_service: web::Data<AuthenticationService>,
) -> ProvidersList {
  let mut providers = authentication_service.list_providers();
  providers.sort();

  ProvidersList {
    providers: providers.into_iter().cloned().collect(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::authentication::ProviderName;
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
