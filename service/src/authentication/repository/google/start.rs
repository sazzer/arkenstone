use super::Provider;
use crate::authentication::repository::ProviderStartAuth;
use uritemplate::UriTemplate;

impl ProviderStartAuth for Provider {
    fn start_auth(&self, nonce: &str) -> String {
        log::info!(
            "Creating authentication URL for Google with Nonce: {} and Settings: {:?}",
            nonce,
            self.settings
        );

        let uri = UriTemplate::new(self.settings.auth_url.as_ref())
            .set("client_id", self.settings.client_id.as_str())
            .set("redirect_uri", self.settings.redirect_url.as_str())
            .set("response_type", "code")
            .set("scope", "openid email profile")
            .set("state", nonce)
            .build();

        log::debug!("Build URI for authentication with Google: {}", uri);

        uri
    }
}
