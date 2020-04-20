use std::collections::HashMap;

/// The health of a single component
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
  Healthy,
  Unhealthy(String),
}

impl ComponentHealth {
  /// Determine if the component is healthy or not
  pub fn is_healthy(&self) -> bool {
    match self {
      ComponentHealth::Healthy => true,
      ComponentHealth::Unhealthy(_) => false,
    }
  }
}

/// The health of the entire system
#[derive(Debug, PartialEq)]
pub struct SystemHealth {
  pub components: HashMap<String, ComponentHealth>,
}

impl SystemHealth {
  /// Construct the health of the system
  pub fn new(components: HashMap<String, ComponentHealth>) -> Self {
    Self { components }
  }

  /// Determine if the system is healthy or not
  pub fn is_healthy(&self) -> bool {
    self
      .components
      .iter()
      .all(|(_, component)| component.is_healthy())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_component_healthy() {
    assert_eq!(true, ComponentHealth::Healthy.is_healthy());
  }

  #[test]
  fn test_component_unhealthy() {
    assert_eq!(
      false,
      ComponentHealth::Unhealthy("Oops".to_owned()).is_healthy()
    );
  }

  #[test]
  fn test_system_healthy_empty() {
    let components = HashMap::new();
    assert_eq!(true, SystemHealth::new(components).is_healthy())
  }

  #[test]
  fn test_system_healthy_passing() {
    let mut components = HashMap::new();
    components.insert("passing".to_owned(), ComponentHealth::Healthy);
    assert_eq!(true, SystemHealth::new(components).is_healthy())
  }

  #[test]
  fn test_system_healthy_failing() {
    let mut components = HashMap::new();
    components.insert(
      "failing".to_owned(),
      ComponentHealth::Unhealthy("Oops".to_owned()),
    );
    assert_eq!(false, SystemHealth::new(components).is_healthy())
  }

  #[test]
  fn test_system_healthy_mixed() {
    let mut components = HashMap::new();
    components.insert("passing".to_owned(), ComponentHealth::Healthy);
    components.insert(
      "failing".to_owned(),
      ComponentHealth::Unhealthy("Oops".to_owned()),
    );
    assert_eq!(false, SystemHealth::new(components).is_healthy())
  }
}
