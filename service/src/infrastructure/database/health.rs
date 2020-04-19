use super::Database;
use crate::infrastructure::health::Component;
use async_trait::async_trait;
use std::boxed::Box;
use std::error::Error;

#[async_trait]
impl Component for Database {
  async fn check_health(&self) -> Result<(), Box<dyn Error>> {
    let connection = self.checkout().await?;
    connection.query("SELECT 1", &[]).await?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::testing::database::TestDatabase;

  #[actix_rt::test]
  async fn test_check_health() {
    let _ = env_logger::try_init();

    let db = TestDatabase::default();

    let database = Database::new(db.url).await;
    database.check_health().await.unwrap();
  }
}
