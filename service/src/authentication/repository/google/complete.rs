use super::{
    claims::{GoogleClaims, GoogleToken},
    Provider,
};
use crate::authentication::repository::{CompleteAuthError, CompletedAuth, ProviderCompleteAuth};
use async_trait::async_trait;
use std::collections::HashMap;
use std::convert::TryInto;

#[async_trait]
impl ProviderCompleteAuth for Provider {
    async fn complete_auth(
        &self,
        params: HashMap<String, String>,
    ) -> Result<CompletedAuth, CompleteAuthError> {
        let auth_code = params.get("code").ok_or_else(|| {
            CompleteAuthError::AuthenticationError("Missing required parameter: code".to_owned())
        })?;

        let client = reqwest::Client::new();

        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", self.settings.client_id.as_ref()),
            ("client_secret", self.settings.client_secret.as_ref()),
            ("redirect_uri", self.settings.redirect_url.as_ref()),
            ("code", auth_code),
        ];
        log::debug!(
            "Making request to URL {} with params {:?}",
            self.settings.token_url,
            params
        );

        let response = client
            .post(&self.settings.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| CompleteAuthError::AuthenticationError(e.to_string()))?;
        log::debug!("Response from Google: {:#?}", response);

        if response.status().as_u16() != 200 {
            let body = response.text().await.unwrap();
            log::warn!("Unsuccessful response received from Google: {}", body);
            return Err(CompleteAuthError::AuthenticationError(
                "Unsuccessful response received from Google".to_owned(),
            ));
        }

        let body: GoogleToken = response
            .json()
            .await
            .map_err(|e| CompleteAuthError::AuthenticationError(e.to_string()))?;
        log::debug!("Response Body from Google: {:?}", body);
        let claims: GoogleClaims = body.try_into()?;
        log::debug!("User Claims from Google: {:?}", claims);

        Ok(claims.into())
    }
}
