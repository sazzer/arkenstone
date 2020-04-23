use crate::authentication::{AuthenticationService, ListProviders, ProviderName};
use actix_web::web;
use serde::Serialize;

/// Actix handler to start authentication with a provider
pub async fn start(
  authentication_service: web::Data<AuthenticationService>,
  path: web::Path<(ProviderName,)>,
) -> String {
  format!("{:?}", path.0)
}
