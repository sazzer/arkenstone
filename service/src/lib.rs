mod infrastructure;

pub async fn main(port: u16) {
  let service = infrastructure::service::Service::new().await;

  service.start(port).await;
}
