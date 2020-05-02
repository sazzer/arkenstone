use serde::{Deserialize, Serialize};

/// New Type to represent the Name of a Player
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayerName(String);
