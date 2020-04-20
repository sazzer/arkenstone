use async_trait::async_trait;
use std::boxed::Box;
use std::error::Error;

/// Any component that is able to report on it's own health may implement this trait to do so
#[async_trait]
pub trait Component {
  /// Check the health of this component
  async fn check_health(&self) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
#[derive(Debug, thiserror::Error, Clone)]
pub enum TestComponent {
  #[error("Not an Error")]
  Healthy,
  #[error("An Error")]
  Unhealthy,
}

#[cfg(test)]
#[async_trait]
impl Component for TestComponent {
  async fn check_health(&self) -> Result<(), Box<dyn Error>> {
    match self {
      TestComponent::Healthy => Ok(()),
      TestComponent::Unhealthy => Err(Box::new(self.clone())),
    }
  }
}
