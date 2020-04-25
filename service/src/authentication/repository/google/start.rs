use super::Provider;
use crate::authentication::repository::ProviderStartAuth;

impl ProviderStartAuth for Provider {
  fn start_auth(&self, nonce: &String) -> String {
    log::info!(
      "Creating authentication URL for Google with Nonce: {}",
      nonce
    );
    todo!()
  }
}
