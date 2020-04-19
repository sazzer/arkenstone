use super::Settings;
use crate::infrastructure::database::{migrate::migrate_database, Database};
use crate::infrastructure::server::Server;

/// The actual service to work with
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new instance of the service
  pub async fn new(settings: Settings) -> Self {
    let db = Database::new(settings.database_url).await;
    migrate_database(&db).await.unwrap();

    let server = Server::new();
    Service { server: server }
  }

  /// Start the service running
  pub async fn start(self, port: u16) {
    self.server.start(port).await;
  }
}
