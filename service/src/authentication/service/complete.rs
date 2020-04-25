use super::AuthenticationService;
#[cfg(test)]
use super::_FauxOriginal_AuthenticationService;
use crate::authentication::{CompleteAuth, CompleteError, ProviderName};
#[cfg(test)]
use faux;
use std::collections::HashMap;

#[cfg_attr(test, faux::methods)]
impl CompleteAuth for AuthenticationService {
  fn complete_auth(
    &self,
    provider: &ProviderName,
    _params: HashMap<String, String>,
  ) -> Result<(), CompleteError> {
    let _provider = self
      .registry
      .find_provider(provider)
      .ok_or(CompleteError::UnknownProvider)?;

    todo!()
  }
}
