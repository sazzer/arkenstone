use super::Settings;

/// Representation of a Provider for authenticating with Google
pub struct Provider {
  _settings: Settings,
}

impl Provider {
  pub fn new(settings: Settings) -> Self {
    Self {
      _settings: settings,
    }
  }
}

impl crate::authentication::repository::Provider for Provider {}
