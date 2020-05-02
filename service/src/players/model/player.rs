use super::{Login, PlayerId, PlayerName};
use crate::model::Model;

/// Representation of the Data that makes up a Player
#[derive(Debug)]
pub struct PlayerData {
    pub name: PlayerName,
    pub avatar_url: Option<String>,
    pub logins: Vec<Login>,
}

/// Representation of a Player Model
pub type PlayerModel = Model<PlayerId, PlayerData>;
