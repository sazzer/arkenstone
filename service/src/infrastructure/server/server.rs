/// The actual HTTP Server
pub struct Server {}

impl Server {
  /// Create the new Server
  pub fn new() -> Self {
    Self {}
  }

  /// Start the HTTP Server listening on the given port
  pub async fn start(self, port: u16) {
    log::debug!("Starting server on port: {}", port);
  }
}
