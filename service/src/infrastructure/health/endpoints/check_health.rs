use super::model::*;
use crate::infrastructure::health::HealthcheckerService;
use actix_web::web;

/// Actix handler to check the health of the system
#[tracing::instrument(name = "GET /health", skip(healthchecker))]
pub async fn check_health(healthchecker: web::Data<HealthcheckerService>) -> SystemHealthModel {
    healthchecker.check_health().await.into()
}
