use crate::authentication::{CompleteError, StartError};
use crate::http::problem::ProblemType;

/// Possible errors when authenticating the user
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AuthProblemType {
    #[error("An Unknown Authentication Provider was requested")]
    UnknownProvider,

    #[error("Failed to authenticate user")]
    AuthenticationError,
}

impl ProblemType for AuthProblemType {
    fn error_code(&self) -> &'static str {
        match self {
            AuthProblemType::UnknownProvider => {
                "tag:arkenstone,2020:authentication/problems/unknown_provider"
            }
            AuthProblemType::AuthenticationError => {
                "tag:arkenstone,2020:authentication/problems/authentication_error"
            }
        }
    }
}

impl From<StartError> for crate::http::problem::Problem<AuthProblemType> {
    fn from(e: StartError) -> Self {
        match e {
            StartError::UnknownProvider => crate::http::problem::Problem::new(
                AuthProblemType::UnknownProvider,
                actix_web::http::StatusCode::NOT_FOUND,
            ),
        }
    }
}

impl From<CompleteError> for crate::http::problem::Problem<AuthProblemType> {
    fn from(e: CompleteError) -> Self {
        match e {
            CompleteError::UnknownProvider => crate::http::problem::Problem::new(
                AuthProblemType::UnknownProvider,
                actix_web::http::StatusCode::NOT_FOUND,
            ),
            CompleteError::AuthenticationError(e) => crate::http::problem::Problem::new(
                AuthProblemType::AuthenticationError,
                actix_web::http::StatusCode::BAD_REQUEST,
            )
            .with_detail(format!("{}", e)),
        }
    }
}
