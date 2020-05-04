use crate::infrastructure::database::Database;

/// Repository used to access Player details from the database
#[derive(Clone)]
pub struct PlayerRepository {
    database: Database,
}

impl PlayerRepository {
    /// Construct a new Player Repository wrapping the given Database connection
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}
