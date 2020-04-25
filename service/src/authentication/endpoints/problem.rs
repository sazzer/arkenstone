use crate::http::problem::ProblemType;

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

impl From<crate::authentication::CompleteError> for crate::http::problem::Problem<AuthProblemType> {
  fn from(e: crate::authentication::CompleteError) -> Self {
    match e {
      crate::authentication::CompleteError::UnknownProvider => crate::http::problem::Problem::new(
        AuthProblemType::UnknownProvider,
        actix_web::http::StatusCode::NOT_FOUND,
      ),
    }
  }
}
