use super::{repository::PlayerRepository, PlayerService};
use crate::infrastructure::database::Database;
use actix_web::web;
use std::sync::Arc;

/// Configurer for the Players System
pub struct PlayersConfig {
    pub player_service: PlayerService,
}

impl PlayersConfig {
    /// Create a new instance of the Players Config
    pub fn new(database: Database) -> Self {
        let player_repository = PlayerRepository::new(database);
        let player_service = PlayerService::new(player_repository);
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
