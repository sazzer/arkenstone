use crate::authentication::repository::Registry;
use crate::players::PlayerService;

/// The actual service to provice Authentication functionality
#[cfg_attr(not(test), derive(Clone))]
#[cfg_attr(test, faux::create)]
pub struct AuthenticationService {
    pub(super) registry: Registry,
    pub(super) player_service: PlayerService,
}

#[cfg_attr(test, faux::methods)]
impl AuthenticationService {
    /// Create a new instance of the Authentication Service
    pub fn new(registry: Registry, player_service: PlayerService) -> Self {
        Self {
            registry,
            player_service,
        }
    }
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Clone for AuthenticationService {
    fn clone(&self) -> Self {
        Self {
            registry: self.registry.clone(),
            player_service: self.player_service.clone(),
        }
    }
}
