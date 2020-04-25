use super::Database;
use rust_embed::RustEmbed;
use thiserror::Error;

/// The embedded migrations files to apply
#[derive(RustEmbed)]
#[folder = "migrations/"]
struct Migrations;

/// Migrate the database schema from the given connection pool to the latest version
pub async fn migrate_database(database: &Database) -> Result<(), MigrationError> {
  log::info!("Migrating database");
  let mut connection = database.checkout().await?;
  let transaction = connection.transaction().await?;

  ensure_migrations_table(&transaction).await?;
  apply_migrations(&transaction).await?;

  transaction.commit().await?;

  Ok(())
}

fn list_all_migrations() -> Result<Vec<String>, MigrationError> {
  log::trace!("Listing all migrations that can be applied");
  let mut migrations: Vec<String> = Migrations::iter().map(|f| f.to_string()).collect();
  migrations.sort();
  log::debug!("All known migrations: {:?}", migrations);

  Ok(migrations)
}

/// Ensure that the migrations table exists and that we've got it locked so that we can proceed with the migration
async fn ensure_migrations_table(
  transaction: &tokio_postgres::Transaction<'_>,
) -> Result<(), MigrationError> {
  log::trace!("Ensuring the migrations table exists");
  transaction
    .execute(
      "CREATE TABLE IF NOT EXISTS __migrations(
            migration_file TEXT PRIMARY KEY,
            sequence SERIAL NOT NULL,
            executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
            executed_from TEXT NOT NULL DEFAULT inet_client_addr()
        )",
      &[],
    )
    .await?;

  log::trace!("Locking the migrations table");
  transaction
    .execute("LOCK TABLE __migrations IN EXCLUSIVE MODE", &[])
    .await?;

  Ok(())
}

/// Get a list of the migrations that have been previously applied
async fn list_applied_migrations(
  transaction: &tokio_postgres::Transaction<'_>,
) -> Result<Vec<String>, MigrationError> {
  log::trace!("Listing the applied migrations");

  let migrations = transaction
    .query("SELECT migration_file FROM __migrations", &[])
    .await?
    .iter()
    .map(|row| row.get::<&str, String>("migration_file"))
    .collect::<Vec<String>>();
  log::debug!("Migrations already applied: {:?}", migrations);

  Ok(migrations)
}

/// Actually apply the migrations that are outstanding
async fn apply_migrations(
  transaction: &tokio_postgres::Transaction<'_>,
) -> Result<(), MigrationError> {
  let all_migrations = list_all_migrations()?;
  let applied_migrations = list_applied_migrations(&transaction).await?;

  let mut count = 0;
  for migration in all_migrations.iter() {
    if applied_migrations.contains(migration) {
      log::debug!("Migration already applied: {}", migration);
    } else {
      log::debug!("Applying migration: {}", migration);
      let contents = Migrations::get(migration).unwrap();

      transaction
        .batch_execute(std::str::from_utf8(&contents)?)
        .await?;
      transaction
        .execute(
          "INSERT INTO __migrations(migration_file) VALUES ($1)",
          &[migration],
        )
        .await?;
      count = count + 1;
    }
  }
  log::info!(
    "Applied {} out of {} migrations",
    count,
    all_migrations.len()
  );

  Ok(())
}

/// Errors that can happen when migrating to the latest schema
#[derive(Error, Debug)]
pub enum MigrationError {
  #[error("Failed to get connection from pool: {0}")]
  DatabasePoolError(#[from] super::DatabaseError),

  #[error("Failed to execute query against database: {0}")]
  DatabaseError(#[from] tokio_postgres::Error),

  #[error("Failed to parse migration file: {0}")]
  FileParseError(#[from] std::str::Utf8Error),
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::testing::database::TestDatabase;

  #[actix_rt::test]
  #[cfg_attr(not(feature = "docker_tests"), ignore)]
  async fn test_migrate() {
    env_logger::try_init().ok();
    let container = TestDatabase::new();
    let database = super::Database::new(container.url).await;

    migrate_database(&database).await.unwrap();
  }

  #[actix_rt::test]
  #[cfg_attr(not(feature = "docker_tests"), ignore)]
  async fn test_migrate_again() {
    env_logger::try_init().ok();
    let container = TestDatabase::new();
    let database = super::Database::new(container.url).await;

    migrate_database(&database).await.unwrap();
    migrate_database(&database).await.unwrap();
  }
}
