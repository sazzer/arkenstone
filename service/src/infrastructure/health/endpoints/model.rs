use crate::infrastructure::health::{ComponentHealth, SystemHealth};
use actix_web::{http::StatusCode, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use std::collections::HashMap;

/// HTTP Representation of the health of a single component
#[derive(Serialize)]
pub struct ComponentHealthModel {
  pub healthy: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
}

/// HTTP Representation of the health of the whole system
#[derive(Serialize)]
pub struct SystemHealthModel {
  pub healthy: bool,
  pub components: HashMap<String, ComponentHealthModel>,
}

impl From<SystemHealth> for SystemHealthModel {
  fn from(health: SystemHealth) -> Self {
    SystemHealthModel {
      healthy: health.is_healthy(),
      components: health
        .components
        .into_iter()
        .map(|(name, component)| {
          let health = ComponentHealthModel {
            healthy: component.is_healthy(),
            message: match component {
              ComponentHealth::Healthy => None,
              ComponentHealth::Unhealthy(msg) => Some(msg),
            },
          };
          (name, health)
        })
        .collect(),
    }
  }
}

impl Responder for SystemHealthModel {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let status_code = if self.healthy {
      StatusCode::OK
    } else {
      StatusCode::SERVICE_UNAVAILABLE
    };

    ready(Ok(HttpResponse::build(status_code).json(self)))
  }
}
