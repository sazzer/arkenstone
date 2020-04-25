/// Configuration Settings for the Google Authentication Provider
#[derive(Debug)]
pub struct Settings {
  pub client_id: String,
  pub auth_url: String,
  pub redirect_url: String,
}

impl From<&crate::Settings> for Option<Settings> {
  fn from(settings: &crate::Settings) -> Self {
    match (&settings.google_client_id, &settings.google_redirect_url) {
      (Some(client_id), Some(redirect_url)) => {
        // Hello
        Some(Settings {
          client_id: client_id.clone(),
          redirect_url: redirect_url.clone(),
          auth_url: settings
            .google_auth_url
            .clone()
            .unwrap_or_else(|| "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_owned()),
        })
      }
      _ => None,
    }
  }
}
