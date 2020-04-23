use super::model::ProviderRedirect;
use crate::authentication::{AuthenticationService, ProviderName};
use actix_web::web;

/// Actix handler to start authentication with a provider
pub async fn start(
  _authentication_service: web::Data<AuthenticationService>,
  path: web::Path<(ProviderName,)>,
) -> ProviderRedirect {
  ProviderRedirect {
    url: "http://www.google.com".to_owned(),
    provider: path.0.clone(),
    nonce: "ThisIsANonce".to_owned(),
  }
}
