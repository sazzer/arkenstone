use super::Settings;

/// Representation of a Provider for authenticating with Google
pub struct Provider {
  pub(super) settings: Settings,
}

impl Provider {
  pub fn new(settings: Settings) -> Self {
    Self { settings: settings }
  }
}

impl crate::authentication::repository::Provider for Provider {}
