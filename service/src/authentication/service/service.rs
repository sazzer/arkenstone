use crate::authentication::repository::Registry;

/// The actual service to provice Authentication functionality
#[cfg_attr(not(test), derive(Clone))]
#[cfg_attr(test, faux::create)]
pub struct AuthenticationService {
  pub(super) registry: Registry,
}

#[cfg_attr(test, faux::methods)]
impl AuthenticationService {
  /// Create a new instance of the Authentication Service
  pub fn new(registry: Registry) -> Self {
    Self { registry }
  }
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Clone for AuthenticationService {
  fn clone(&self) -> Self {
    Self {
      registry: self.registry.clone(),
    }
  }
}
