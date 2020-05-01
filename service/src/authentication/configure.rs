use crate::authentication::{
    endpoints,
    repository::{google, RegistryBuilder},
    AuthenticationService,
};
use crate::players::PlayerService;
use actix_web::web;
use std::sync::Arc;

/// Configurer for the Authentication System
pub struct AuthenticationConfig {
    registry_builder: RegistryBuilder,
    player_service: PlayerService,
}

impl AuthenticationConfig {
    pub fn new(player_service: PlayerService) -> Self {
        Self {
            registry_builder: RegistryBuilder::default(),
            player_service,
        }
    }

    /// Attempt to register a Google Authentication Provider
    pub fn with_google(mut self, settings: Option<google::Settings>) -> Self {
        if let Some(settings) = settings {
            self.registry_builder
                .with_provider("google", Arc::new(google::Provider::new(settings)));
        }
        self
    }

    /// Build the configuration functor for the server
    pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
        let registry = self.registry_builder.build();
        let service = AuthenticationService::new(registry, self.player_service.clone());
        Arc::new(move |config| {
            config.data(service.clone());
            config.route("/authentication", web::get().to(endpoints::list_providers));
            config.route(
                "/authentication/{provider}",
                web::get().to(endpoints::start),
            );
            config.route(
                "/authentication/{provider}/complete",
                web::get().to(endpoints::complete),
            );
        })
    }
}
