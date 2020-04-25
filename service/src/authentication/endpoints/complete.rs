use super::problem::AuthProblemType;
use crate::authentication::{AuthenticationService, ProviderName};
use crate::http::problem::Problem;
use actix_web::{web, HttpRequest};
use std::collections::HashMap;
use url;

/// Actix handler to complete authentication with a provider
pub async fn complete(
  _authentication_service: web::Data<AuthenticationService>,
  req: HttpRequest,
  path: web::Path<(ProviderName,)>,
) -> Result<String, Problem<AuthProblemType>> {
  let params: HashMap<String, String> = req
    .uri()
    .query()
    .map(|v| {
      url::form_urlencoded::parse(v.as_bytes())
        .into_owned()
        .collect()
    })
    .unwrap_or_else(HashMap::new);

  log::info!(
    "Completing authentication with provider {:?} and parameters {:?}",
    path.0,
    params
  );

  Ok("Hello".to_owned())
}
