use super::Provider;
use crate::authentication::repository::ProviderStartAuth;

impl ProviderStartAuth for Provider {
  fn start_auth(&self, _nonce: String) -> String {
    todo!()
  }
}
