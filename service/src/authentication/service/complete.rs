use super::AuthenticationService;
use crate::authentication::{CompleteAuth, CompleteError};
use crate::players::{ProviderName, RegisterUser, UserRegistration};
use async_trait::async_trait;
use std::collections::HashMap;

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

        let registration = UserRegistration {
            external_system: provider_name.clone(),
            external_id: user_details.external_id,
            display_name: user_details.display_name,
            name: user_details.name,
            avatar_url: user_details.avatar_url,
        };

        self.player_service
            .register_user(registration)
            .await
            .unwrap();

        Ok(())
    }
}
