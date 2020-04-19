use async_trait::async_trait;
use std::boxed::Box;
use std::error::Error;

/// Any component that is able to report on it's own health may implement this trait to do so
#[async_trait]
pub trait Component {
  /// Check the health of this component
  async fn check_health(&self) -> Result<(), Box<dyn Error>>;
}
