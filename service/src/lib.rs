mod authentication;
mod http;
mod infrastructure;
mod settings;
pub mod testing;

pub use infrastructure::server::testing::TestResponse;
pub use infrastructure::service::Service;
pub use settings::Settings;

pub async fn main(settings: Settings, port: u16) {
  let service = Service::new(&settings).await;

  service.start(port).await;
}
