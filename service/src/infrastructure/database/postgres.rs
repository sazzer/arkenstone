use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;

/// Interface to making database calls
pub struct Database {
  _pool: Pool<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
}

impl Database {
  /// Create the new database wrapper
  pub async fn new<S>(url: S) -> Self
  where
    S: Into<String>,
  {
    let url = url.into();
    log::info!("Connecting to database: {}", url);

    let config = tokio_postgres::config::Config::from_str(&url).unwrap();
    let manager = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

    let pool = Pool::builder()
      .connection_timeout(std::time::Duration::from_secs(10))
      .build(manager)
      .await
      .unwrap();

    pool.get().await.unwrap();
    Self { _pool: pool }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::testing::database::TestDatabase;

  #[actix_rt::test]
  async fn test_database_connection() {
    let _ = env_logger::try_init();

    let db = TestDatabase::default();

    Database::new(db.url).await;
  }
}
