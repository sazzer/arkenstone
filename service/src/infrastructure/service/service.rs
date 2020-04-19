/// The actual service to work with
pub struct Service {}

impl Service {
  /// Construct a new instance of the service
  pub async fn new() -> Self {
    Service {}
  }

  /// Start the service running
  pub async fn start(self, port: u16) {
    log::debug!("Starting service on port {}", port);
  }
}
