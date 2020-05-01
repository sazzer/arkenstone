use actix_web::web;
use std::sync::Arc;

/// Configurer for the Players System
#[derive(Default)]
pub struct PlayersConfig {}

impl PlayersConfig {
    pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
        Arc::new(move |_config| {})
    }
}
