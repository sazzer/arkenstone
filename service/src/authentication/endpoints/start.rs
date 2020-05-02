use super::problem::AuthProblemType;
use crate::authentication::{AuthenticationService, StartAuth};
use crate::http::problem::Problem;
use crate::players::ProviderName;
use actix_web::{http::Cookie, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

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
        ready(Ok(HttpResponse::Found()
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
            .finish()))
    }
}

/// Actix handler to start authentication with a provider
pub async fn start(
    authentication_service: web::Data<AuthenticationService>,
    path: web::Path<(ProviderName,)>,
) -> Result<ProviderRedirect, Problem<AuthProblemType>> {
    let response = authentication_service.start_auth(&path.0)?;

    Ok(ProviderRedirect {
        url: response.url,
        provider: path.0.clone(),
        nonce: response.nonce,
    })
}
