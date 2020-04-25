use crate::authentication::{
  endpoints,
  repository::{google, RegistryBuilder},
  AuthenticationService,
};
use actix_web::web;
use std::sync::Arc;

/// Configurer for the Authentication System
#[derive(Default)]
pub struct AuthenticationConfig {
  registry_builder: RegistryBuilder,
}

impl AuthenticationConfig {
  /// Attempt to register a Google Authentication Provider
  pub fn with_google(mut self, settings: Option<google::Settings>) -> Self {
    if let Some(settings) = settings {
      self
        .registry_builder
        .with_provider("google", Arc::new(google::Provider::new(settings)));
    }
    self
  }

  /// Build the configuration functor for the server
  pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
    let registry = self.registry_builder.build();
    let service = AuthenticationService::new(registry);
    Arc::new(move |config| {
      config.data(service.clone());
      config.route("/authentication", web::get().to(endpoints::list_providers));
      config.route(
        "/authentication/{provider}",
        web::get().to(endpoints::start),
      );
    })
  }
}
