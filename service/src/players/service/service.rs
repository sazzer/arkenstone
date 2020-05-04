use crate::players::repository::PlayerRepository;

/// The actual service to provide Player functionality
#[derive(Clone)]
pub struct PlayerService {
    pub(super) repository: PlayerRepository,
}

impl PlayerService {
    /// Create a new instance of the Player Service
    pub fn new(repository: PlayerRepository) -> Self {
        Self { repository }
    }
}
