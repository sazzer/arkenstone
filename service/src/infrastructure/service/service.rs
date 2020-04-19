/// The actual service to work with
pub struct Service {}

impl Service {
  /// Construct a new instance of the service
  pub fn new() -> Self {
    Service {}
  }

  /// Start the service running
  pub fn start(self, port: u16) {
    log::debug!("Starting service on port {}", port);
  }
}
