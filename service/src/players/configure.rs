use super::PlayerService;
use actix_web::web;
use std::sync::Arc;

/// Configurer for the Players System
pub struct PlayersConfig {
    pub player_service: PlayerService,
}

impl PlayersConfig {
    /// Create a new instance of the Players Config
    pub fn new() -> Self {
        let player_service = PlayerService::new();
        Self { player_service }
    }
}

impl PlayersConfig {
    pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
        let service = self.player_service.clone();
        Arc::new(move |config| {
            config.data(service.clone());
        })
    }
}
