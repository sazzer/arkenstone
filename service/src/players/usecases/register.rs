use crate::players::{PlayerModel, PlayerService, ProviderName, ProviderPlayerId};

/// Details needed to register a new user
#[derive(Debug)]
pub struct UserRegistration {
    pub external_system: ProviderName,
    pub external_id: ProviderPlayerId,
    pub display_name: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

impl PlayerService {
    pub async fn register_user(&self, registration: UserRegistration) -> Result<PlayerModel, ()> {
        log::debug!("Registering user with details: {:?}", registration);

        // Look up user by external_system and external_id
        let player = self
            .repository
            .find_player_by_external_id(&registration.external_system, &registration.external_id)
            // If no user is found, create a new one and then return this
            .unwrap_or_else(|| todo!());

        // If user is found, return this as-is

        Ok(player)
    }
}
