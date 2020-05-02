use super::AuthenticationService;
use crate::authentication::{repository::CompletedAuth, CompleteAuth, CompleteError, ProviderName};
use crate::players::{RegisterUser, UserRegistration};
use async_trait::async_trait;
use std::collections::HashMap;

impl From<CompletedAuth> for UserRegistration {
    fn from(auth: CompletedAuth) -> Self {
        Self {
            external_system: "google".to_owned(),
            external_id: auth.external_id,
            display_name: auth.display_name,
            name: auth.name,
            avatar_url: auth.avatar_url,
        }
    }
}

#[async_trait]
impl CompleteAuth for AuthenticationService {
    async fn complete_auth(
        &self,
        provider_name: &ProviderName,
        params: HashMap<String, String>,
    ) -> Result<(), CompleteError> {
        let provider = self
            .registry
            .find_provider(provider_name)
            .ok_or(CompleteError::UnknownProvider)?;

        let user_details = provider.complete_auth(params).await?;
        log::info!(
            "Authenticated as user {:?} with provider {:?}",
            user_details,
            provider_name
        );

        self.player_service
            .register_user(user_details.into())
            .await
            .unwrap();

        Ok(())
    }
}
