use super::AuthenticationService;
#[cfg(test)]
use super::_FauxOriginal_AuthenticationService;
use crate::authentication::{CompleteAuth, CompleteError, ProviderName};
use std::collections::HashMap;

#[cfg_attr(test, faux::methods)]
impl CompleteAuth for AuthenticationService {
  fn complete_auth(
    &self,
    provider_name: &ProviderName,
    _params: HashMap<String, String>,
  ) -> Result<(), CompleteError> {
    let _provider = self
      .registry
      .find_provider(provider_name)
      .ok_or(CompleteError::UnknownProvider)?;

    todo!()
  }
}
