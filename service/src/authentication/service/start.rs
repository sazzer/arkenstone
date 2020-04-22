use super::AuthenticationService;
use crate::authentication::{ProviderName, StartAuth, StartAuthDetails};

impl StartAuth for AuthenticationService {
  fn start_auth(&self, _provider: ProviderName) -> StartAuthDetails {
    todo!()
  }
}
