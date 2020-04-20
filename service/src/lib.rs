mod infrastructure;
pub use infrastructure::server::testing::TestResponse;
pub use infrastructure::service::{Service, Settings};
pub mod testing;

pub async fn main(settings: Settings, port: u16) {
  let service = Service::new(settings).await;

  service.start(port).await;
}
