/// The actual service to provide Player functionality
#[cfg_attr(not(test), derive(Clone))]
#[cfg_attr(test, faux::create)]
pub struct PlayerService {}

#[cfg_attr(test, faux::methods)]
impl PlayerService {
    /// Create a new instance of the Player Service
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Clone for PlayerService {
    fn clone(&self) -> Self {
        Self {}
    }
}
