/// Configuration Settings for the Google Authentication Provider
#[derive(Debug)]
pub struct Settings {
  pub client_id: String,
  pub client_secret: String,
  pub auth_url: String,
  pub token_url: String,
  pub redirect_url: String,
}

impl From<&crate::Settings> for Option<Settings> {
  fn from(settings: &crate::Settings) -> Self {
    match (
      &settings.google_client_id,
      &settings.google_client_secret,
      &settings.google_redirect_url,
    ) {
      (Some(client_id), Some(client_secret), Some(redirect_url)) => {
        // Hello
        Some(Settings {
          client_id: client_id.clone(),
          client_secret: client_secret.clone(),
          redirect_url: redirect_url.clone(),
          auth_url: settings
            .google_auth_url
            .clone()
            .unwrap_or_else(|| "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_owned()),
          token_url: settings
            .google_token_url
            .clone()
            .unwrap_or_else(|| "https://www.googleapis.com/oauth2/v4/token".to_owned()),
        })
      }
      _ => None,
    }
  }
}
