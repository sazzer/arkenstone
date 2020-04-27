use super::Provider;
use crate::authentication::repository::ProviderCompleteAuth;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
impl ProviderCompleteAuth for Provider {
  async fn complete_auth(&self, params: HashMap<String, String>) -> Result<(), ()> {
    todo!()
  }
}
