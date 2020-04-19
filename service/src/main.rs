use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// The environmental settings that are used for running the app
#[derive(Debug, Deserialize)]
struct Settings {
    port: Option<u16>,
    database_url: String,
}

impl Settings {
    /// Construct the settings from the environment
    fn new() -> Settings {
        let mut s = Config::new();
        s.merge(Environment::default()).unwrap();

        s.try_into().unwrap()
    }
}

#[actix_rt::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let settings = Settings::new();
    log::debug!("Application settings: {:?}", settings);

    arkenstone_lib::main(
        arkenstone_lib::Settings {
            database_url: settings.database_url,
        },
        settings.port.unwrap_or(8000),
    )
    .await
}
