use crate::players::PlayerModel;
use async_trait::async_trait;

/// Details needed to register a new user
#[derive(Debug)]
pub struct UserRegistration {
    pub external_system: String,
    pub external_id: String,
    pub display_name: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

/// Trait allowing for registration of a user
#[async_trait]
pub trait RegisterUser {
    /// Actually attempt to register a new user
    async fn register_user(&self, registration: UserRegistration) -> Result<PlayerModel, ()>;
}
