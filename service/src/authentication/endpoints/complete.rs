use super::problem::AuthProblemType;
use crate::authentication::AuthenticationService;
use crate::http::problem::Problem;
use crate::players::ProviderName;
use actix_web::{web, HttpRequest};
use std::collections::HashMap;

/// Actix handler to complete authentication with a provider
#[tracing::instrument(
    name = "GET /authentication/:provider/complete",
    skip(authentication_service, req)
)]
pub async fn complete(
    authentication_service: web::Data<AuthenticationService>,
    req: HttpRequest,
    path: web::Path<(ProviderName,)>,
) -> Result<String, Problem<AuthProblemType>> {
    let params: HashMap<String, String> = req.uri().query().map_or_else(HashMap::new, |v| {
        url::form_urlencoded::parse(v.as_bytes())
            .into_owned()
            .collect()
    });

    log::info!(
        "Completing authentication with provider {:?} and parameters {:?}",
        path.0,
        params
    );

    authentication_service
        .complete_auth(&path.0, params)
        .await?;
    Ok("Hello".to_owned())
}
