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
