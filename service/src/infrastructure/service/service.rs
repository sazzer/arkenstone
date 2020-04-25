use crate::authentication::configure::AuthenticationConfig;
use crate::infrastructure::database::{migrate::migrate_database, Database};
use crate::infrastructure::health::configure::HealthcheckConfig;
use crate::infrastructure::server::Server;
use crate::Settings;
use actix_http::Request;
use std::sync::Arc;

/// The actual service to work with
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new instance of the service
  pub async fn new(settings: &Settings) -> Self {
    let db = Database::new(settings.database_url.clone()).await;
    migrate_database(&db).await.unwrap();

    let healthchecker = HealthcheckConfig::default().with_component("db", Arc::new(db));
    let authentication = AuthenticationConfig::default().with_google(settings.into());

    let server = Server::new(vec![healthchecker.configure(), authentication.configure()]);
    Self { server }
  }

  /// Start the service running
  pub async fn start(self, port: u16) {
    self.server.start(port).await;
  }

  /// Submit a request to the server and return the response
  pub async fn test_request(
    &self,
    req: Request,
  ) -> crate::infrastructure::server::testing::TestResponse {
    self.server.test_request(req).await
  }
}
