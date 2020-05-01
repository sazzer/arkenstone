use crate::players::{PlayerService, RegisterUser, UserRegistration};
use async_trait::async_trait;

#[async_trait]
impl RegisterUser for PlayerService {
    async fn register_user(&self, registration: UserRegistration) -> Result<(), ()> {
        log::debug!("Registering user with details: {:?}", registration);
        Ok(())
    }
}
