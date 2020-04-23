use crate::authentication::ProviderName;
use actix_web::{http::StatusCode, Error, HttpRequest, HttpResponse, Responder};
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
    ready(Ok(HttpResponse::build(StatusCode::OK).json(self)))
  }
}
