use super::Settings;
use crate::infrastructure::database::Database;
use crate::infrastructure::server::Server;

/// The actual service to work with
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new instance of the service
  pub async fn new(settings: Settings) -> Self {
    let _db = Database::new(settings.database_url).await;

    let server = Server::new();
    Service { server: server }
  }

  /// Start the service running
  pub async fn start(self, port: u16) {
    self.server.start(port).await;
  }
}
