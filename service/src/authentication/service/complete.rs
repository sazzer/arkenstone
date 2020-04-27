use super::AuthenticationService;
#[cfg(test)]
use super::_FauxOriginal_AuthenticationService;
use crate::authentication::{CompleteAuth, CompleteError, ProviderName};
use async_trait::async_trait;
use std::collections::HashMap;

#[cfg_attr(test, faux::methods)]
#[async_trait]
impl CompleteAuth for AuthenticationService {
  async fn complete_auth(
    &self,
    provider_name: &ProviderName,
    params: HashMap<String, String>,
  ) -> Result<(), CompleteError> {
    let provider = self
      .registry
      .find_provider(provider_name)
      .ok_or(CompleteError::UnknownProvider)?;

    provider.complete_auth(params).await.unwrap();

    Ok(())
  }
}
