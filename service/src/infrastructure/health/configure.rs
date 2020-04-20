use super::{endpoints, Component, HealthcheckerServiceBuilder};
use actix_web::web;
use std::sync::Arc;

/// Configurer for the Healthcheckes
#[derive(Default)]
pub struct HealthcheckConfig {
  builder: HealthcheckerServiceBuilder,
}

impl HealthcheckConfig {
  /// Add a new component to the health checker that we are building
  pub fn with_component<S>(mut self, name: S, component: Arc<dyn Component>) -> Self
  where
    S: Into<String>,
  {
    self.builder.with_component(name, component);

    self
  }
  /// Build the configuration functor for the server
  pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
    let healthchecker = self.builder.build();

    Arc::new(move |config| {
      config.data(healthchecker.clone());
      config.route("/health", web::get().to(endpoints::check_health));
    })
  }
}
