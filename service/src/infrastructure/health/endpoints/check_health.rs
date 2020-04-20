use super::model::*;
use crate::infrastructure::health::HealthcheckerService;
use actix_web::web;

/// Actix handler to check the health of the system
pub async fn check_health(healthchecker: web::Data<HealthcheckerService>) -> SystemHealthModel {
  healthchecker.check_health().await.into()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::infrastructure::health::{HealthcheckerServiceBuilder, TestComponent};
  use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
  };
  use std::sync::Arc;

  #[actix_rt::test]
  async fn test_no_components() {
    let healthchecker = HealthcheckerServiceBuilder::default().build();

    let result = check_health(web::Data::new(healthchecker)).await;
    assert_that!(&result.healthy, eq(true));
    assert_that!(&result.components.len(), eq(0));
  }

  #[actix_rt::test]
  async fn test_healthy_component() {
    let healthchecker = HealthcheckerServiceBuilder::default()
      .with_component("passing", Arc::new(TestComponent::Healthy))
      .build();

    let result = check_health(web::Data::new(healthchecker)).await;
    assert_that!(&result.healthy, eq(true));
    assert_that!(&result.components.len(), eq(1));
    assert_that!(&result.components.get("passing").unwrap().healthy, eq(true));
    assert_that!(&result.components.get("passing").unwrap().message, eq(None));
  }

  #[actix_rt::test]
  async fn test_unhealthy_component() {
    let healthchecker = HealthcheckerServiceBuilder::default()
      .with_component("failing", Arc::new(TestComponent::Unhealthy))
      .build();

    let result = check_health(web::Data::new(healthchecker)).await;
    assert_that!(&result.healthy, eq(false));
    assert_that!(&result.components.len(), eq(1));
    assert_that!(
      &result.components.get("failing").unwrap().healthy,
      eq(false)
    );
    assert_that!(
      &result.components.get("failing").unwrap().message,
      maybe_some(eq("An Error".to_owned()))
    );
  }

  #[actix_rt::test]
  async fn test_mixed_components() {
    let healthchecker = HealthcheckerServiceBuilder::default()
      .with_component("failing", Arc::new(TestComponent::Unhealthy))
      .with_component("passing", Arc::new(TestComponent::Healthy))
      .build();

    let result = check_health(web::Data::new(healthchecker)).await;
    assert_that!(&result.healthy, eq(false));
    assert_that!(&result.components.len(), eq(2));
    assert_that!(&result.components.get("passing").unwrap().healthy, eq(true));
    assert_that!(&result.components.get("passing").unwrap().message, eq(None));
    assert_that!(
      &result.components.get("failing").unwrap().healthy,
      eq(false)
    );
    assert_that!(
      &result.components.get("failing").unwrap().message,
      maybe_some(eq("An Error".to_owned()))
    );
  }
}
