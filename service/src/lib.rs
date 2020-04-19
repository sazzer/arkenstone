mod infrastructure;

pub fn main(port: u16) {
  log::info!("Hello, world!");

  let service = infrastructure::service::Service::new();

  service.start(port);
}
