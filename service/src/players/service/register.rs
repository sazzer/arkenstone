use crate::players::{PlayerModel, PlayerService, RegisterUser, UserRegistration};
use async_trait::async_trait;

#[async_trait]
impl RegisterUser for PlayerService {
    async fn register_user(&self, registration: UserRegistration) -> Result<PlayerModel, ()> {
        log::debug!("Registering user with details: {:?}", registration);

        // Look up user by external_system and external_id

        // If no user is found, create a new one and then return this
        // If user is found, return this as-is

        todo!()
    }
}
