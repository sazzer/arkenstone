use crate::authentication::ProviderName;
use actix_web::{http::Cookie, Error, HttpRequest, HttpResponse, Responder};
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

/// API Model to represent a redirect to start authenticating with a provider
pub struct ProviderRedirect {
  pub url: String,
  pub provider: ProviderName,
  pub nonce: String,
}

impl Responder for ProviderRedirect {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    ready(Ok(
      HttpResponse::Found()
        .header("Location", self.url)
        .cookie(
          Cookie::build("authentication_provider", self.provider)
            .http_only(true)
            .finish(),
        )
        .cookie(
          Cookie::build("authentication_nonce", self.nonce)
            .http_only(true)
            .finish(),
        )
        .finish(),
    ))
  }
}
