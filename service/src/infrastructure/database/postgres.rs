use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;

/// Interface to making database calls
#[derive(Clone)]
pub struct Database {
    pool: Pool<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
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
        Self { pool }
    }

    /// Check out a connection with which we can send queries to the database
    pub async fn checkout(
        &self,
    ) -> Result<
        PooledConnection<'_, PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
        DatabaseError,
    > {
        self.pool
            .get()
            .await
            .map_err(|e| DatabaseError::CheckoutError(format!("{}", e)))
    }
}

/// Errors that can happen when working with the database
#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Error checking out connection: {0}")]
    CheckoutError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::database::TestDatabase;

    #[actix_rt::test]
    #[cfg_attr(not(feature = "docker_tests"), ignore)]
    async fn test_database_connection() {
        let _ = env_logger::try_init();

        let db = TestDatabase::default();

        Database::new(db.url).await;
    }

    #[actix_rt::test]
    #[cfg_attr(not(feature = "docker_tests"), ignore)]
    async fn test_checkout() {
        let _ = env_logger::try_init();

        let db = TestDatabase::default();

        let database = Database::new(db.url).await;
        let conn = database.checkout().await.unwrap();

        conn.query("SELECT 1", &[]).await.unwrap();
    }
}
