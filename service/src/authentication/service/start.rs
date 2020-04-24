use super::AuthenticationService;
#[cfg(test)]
use super::_FauxOriginal_AuthenticationService;
use crate::authentication::{ProviderName, StartAuth, StartAuthDetails};
#[cfg(test)]
use faux;

#[cfg_attr(test, faux::methods)]
impl StartAuth for AuthenticationService {
  fn start_auth(&self, _provider: &ProviderName) -> StartAuthDetails {
    todo!()
  }
}
