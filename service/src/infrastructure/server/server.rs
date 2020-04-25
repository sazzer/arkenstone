use super::testing::TestResponse;
use actix_cors::Cors;
use actix_http::Request;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::ops::Deref;
use std::sync::Arc;

/// The type used to represent a function to provide configuration to Actix
pub type FnConfig = Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync>;

/// The actual HTTP Server
pub struct Server {
  configs: Vec<FnConfig>,
}

impl Server {
  /// Create the new Server
  pub fn new(configs: Vec<FnConfig>) -> Self {
    log::info!("Creating HTTP Server");

    Self { configs }
  }

  /// Start the HTTP Server listening on the given port
  pub async fn start(self, port: u16) {
    let configs = self.configs.clone();
    let builder = move || {
      let configs = configs.clone();
      let mut app = App::new()
        .wrap(Logger::default())
        .wrap(Cors::new().finish());
      for config in &configs {
        app = app.configure(config.deref());
      }
      app
    };

    let listen_address = format!("0.0.0.0:{}", port);
    log::info!("Starting web server on {}", listen_address);
    HttpServer::new(builder)
      .bind(listen_address)
      .unwrap()
      .run()
      .await
      .unwrap();
  }

  /// Submit a request to the server and return the response
  pub async fn test_request(&self, request: Request) -> TestResponse {
    let mut app = App::new()
      .wrap(Logger::default())
      .wrap(Cors::new().finish());
    for config in &self.configs {
      app = app.configure(config.deref());
    }

    let mut app = actix_web::test::init_service(app).await;
    let response = actix_web::test::call_service(&mut app, request).await;

    let status = response.status();
    let headers = response.headers().clone();
    let body = actix_web::test::read_body(response).await;
    TestResponse {
      status,
      headers,
      body,
    }
  }
}
