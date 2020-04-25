use super::Provider;
use crate::authentication::repository::ProviderStartAuth;
use uritemplate::UriTemplate;

impl ProviderStartAuth for Provider {
  fn start_auth(&self, nonce: &String) -> String {
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
      .set("state", nonce.as_str())
      .build();

    log::debug!("Build URI for authentication with Google: {}", uri);

    uri
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::authentication::repository::google::Settings;

  #[test]
  fn test_start_auth() {
    let sut = Provider::new(Settings {
      client_id: "someClientId".to_owned(),
      redirect_url: "http://localhost:8080/authentication/google/complete".to_owned(),
      auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_owned(),
    });

    let result = sut.start_auth(&"someNonce".to_owned());

    assert_eq!(
      "https://accounts.google.com/o/oauth2/v2/auth?client_id=someClientId&response_type=code&scope=openid%20email%20profile&redirect_uri=http%3A%2F%2Flocalhost%3A8080%2Fauthentication%2Fgoogle%2Fcomplete&state=someNonce".to_owned(), 
      result);
  }
}
