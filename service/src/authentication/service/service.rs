use crate::authentication::repository::Registry;

/// The actual service to provice Authentication functionality
#[derive(Clone)]
pub struct AuthenticationService {
  pub(super) registry: Registry,
}

impl AuthenticationService {
  /// Create a new instance of the Authentication Service
  pub fn new(registry: Registry) -> Self {
    Self { registry }
  }
}
