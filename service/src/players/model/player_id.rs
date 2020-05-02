use serde::{Deserialize, Serialize};

/// New Type to represent the ID of a Player
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayerId(String);
