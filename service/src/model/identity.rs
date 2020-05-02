use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Structure to represent the identity of some model in the data store
#[derive(Debug, PartialEq, Clone)]
pub struct Identity<ID> {
    pub id: ID,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl<ID> Default for Identity<ID>
where
    ID: Default,
{
    fn default() -> Self {
        Self {
            id: ID::default(),
            version: Uuid::new_v4(),
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}
