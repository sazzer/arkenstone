mod infrastructure;
pub use infrastructure::service::Settings;

pub async fn main(settings: Settings, port: u16) {
  let service = infrastructure::service::Service::new(settings).await;

  service.start(port).await;
}
