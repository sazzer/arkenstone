use crate::authentication::ProviderName;
use crate::http::problem::ProblemType;
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
#[derive(Debug)]
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

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AuthProblemType {
  #[error("An Unknown Authentication Provider was requested")]
  UnknownProvider,
}

impl ProblemType for AuthProblemType {
  fn error_code(&self) -> &'static str {
    match self {
      AuthProblemType::UnknownProvider => {
        "tag:arkenstone,2020:authentication/problems/unknown_provider"
      }
    }
  }
}

impl From<crate::authentication::StartError> for crate::http::problem::Problem<AuthProblemType> {
  fn from(e: crate::authentication::StartError) -> Self {
    match e {
      crate::authentication::StartError::UnknownProvider => crate::http::problem::Problem::new(
        AuthProblemType::UnknownProvider,
        actix_web::http::StatusCode::NOT_FOUND,
      ),
    }
  }
}
