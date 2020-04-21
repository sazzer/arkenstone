use actix_web::web;
use std::sync::Arc;

/// Configurer for the Authentication System
#[derive(Default)]
pub struct AuthenticationConfig {}

impl AuthenticationConfig {
  /// Build the configuration functor for the server
  pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
    Arc::new(|_config| {})
  }
}
