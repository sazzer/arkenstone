#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::module_name_repetitions,
        clippy::module_inception,
        clippy::used_underscore_binding,
        clippy::wildcard_imports,
        clippy::missing_errors_doc
    )
)]

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
