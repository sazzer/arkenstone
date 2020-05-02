use crate::authentication::repository::Registry;
use crate::players::PlayerService;

/// The actual service to provice Authentication functionality
#[derive(Clone)]
pub struct AuthenticationService {
    pub(super) registry: Registry,
    pub(super) player_service: PlayerService,
}

impl AuthenticationService {
    /// Create a new instance of the Authentication Service
    pub fn new(registry: Registry, player_service: PlayerService) -> Self {
        Self {
            registry,
            player_service,
        }
    }
}

