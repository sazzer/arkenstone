use serde::{Deserialize, Serialize};

/// New Type to represent the ID of a Player at an external Provider
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProviderPlayerId(String);
