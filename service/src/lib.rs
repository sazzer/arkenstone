mod authentication;
mod infrastructure;
pub mod testing;

pub use infrastructure::server::testing::TestResponse;
pub use infrastructure::service::{Service, Settings};

pub async fn main(settings: Settings, port: u16) {
  let service = Service::new(settings).await;

  service.start(port).await;
}
