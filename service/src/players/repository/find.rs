use crate::players::{repository::PlayerRepository, PlayerModel, ProviderName, ProviderPlayerId};

impl PlayerRepository {
    /// Find the player that has the provided ID at the provided Authentication Provider
    pub fn find_player_by_external_id(
        &self,
        _external_provider: &ProviderName,
        _external_id: &ProviderPlayerId,
    ) -> Option<PlayerModel> {
        None
    }
}
