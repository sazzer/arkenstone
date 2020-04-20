use crate::infrastructure::health::{Component, ComponentHealth, SystemHealth};
use std::collections::HashMap;
use std::sync::Arc;

/// The actual service layer that can check the health of the system
pub struct HealthcheckerService {
  components: HashMap<String, Arc<dyn Component>>,
}

impl HealthcheckerService {
  /// Create a new instance of the health checker service
  pub fn new(components: HashMap<String, Arc<dyn Component>>) -> Self {
    Self { components }
  }

  /// Actually check the health of the system
  pub async fn check_health(&self) -> SystemHealth {
    let mut results = HashMap::new();

    for (name, component) in self.components.iter() {
      let result = component.check_health().await;

      results.insert(
        name.clone(),
        match result {
          Ok(_) => ComponentHealth::Healthy,
          Err(e) => ComponentHealth::Unhealthy(format!("{}", e)),
        },
      );
    }

    SystemHealth::new(results)
  }
}

/// Builder for the service layer that can check the health of the system
#[derive(Default)]
pub struct HealthcheckerServiceBuilder {
  components: HashMap<String, Arc<dyn Component>>,
}

impl HealthcheckerServiceBuilder {
  /// Add a new component to the health checker that we are building
  pub fn with_component<S>(mut self, name: S, component: Arc<dyn Component>) -> Self
  where
    S: Into<String>,
  {
    self.components.insert(name.into(), component);

    self
  }

  /// Actually build the health checker
  pub fn build(self) -> HealthcheckerService {
    HealthcheckerService::new(self.components)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::infrastructure::health::TestComponent;
  use galvanic_assert::{
    assert_that,
    matchers::{collection::*, *},
  };

  #[actix_rt::test]
  async fn test_check_health_empty() {
    let sut = HealthcheckerServiceBuilder::default().build();

    let result = sut.check_health().await;
    assert_that!(&result.components.len(), eq(0));
  }

  #[actix_rt::test]
  async fn test_check_health_passing() {
    let sut = HealthcheckerServiceBuilder::default()
      .with_component("passing", Arc::new(TestComponent::Healthy))
      .build();

    let result = sut.check_health().await;
    assert_that!(&result.components.len(), eq(1));
    assert_that!(
      &result.components,
      has_entry("passing".to_owned(), ComponentHealth::Healthy)
    );
  }

  #[actix_rt::test]
  async fn test_check_health_failing() {
    let sut = HealthcheckerServiceBuilder::default()
      .with_component("failing", Arc::new(TestComponent::Unhealthy))
      .build();

    let result = sut.check_health().await;
    assert_that!(&result.components.len(), eq(1));
    assert_that!(
      &result.components,
      has_entry(
        "failing".to_owned(),
        ComponentHealth::Unhealthy("An Error".to_owned())
      )
    );
  }

  #[actix_rt::test]
  async fn test_check_health_mixed() {
    let sut = HealthcheckerServiceBuilder::default()
      .with_component("failing", Arc::new(TestComponent::Unhealthy))
      .with_component("passing", Arc::new(TestComponent::Healthy))
      .build();

    let result = sut.check_health().await;
    assert_that!(&result.components.len(), eq(2));
    assert_that!(
      &result.components,
      has_entry("passing".to_owned(), ComponentHealth::Healthy)
    );
    assert_that!(
      &result.components,
      has_entry(
        "failing".to_owned(),
        ComponentHealth::Unhealthy("An Error".to_owned())
      )
    );
  }
}
