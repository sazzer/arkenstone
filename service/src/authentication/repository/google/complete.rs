use super::Provider;
use crate::authentication::repository::ProviderCompleteAuth;
use async_trait::async_trait;
use jsonwebtoken;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

/// The shape of the Token details retrieved from Google
#[derive(Debug, Deserialize)]
pub struct GoogleToken {
  id_token: Option<String>,
}

/// The shape of the claims inside the ID Token from Google
#[derive(Debug, Deserialize)]
pub struct GoogleClaims {
  sub: String,
  email: Option<String>,
  name: Option<String>,
  picture: Option<String>,
}

/// The errors that can occur when decoding the ID Token
#[derive(Debug, thiserror::Error)]
pub enum ClaimsError {
  #[error("No ID Token was present")]
  MissingIdToken,

  #[error("An error occurred decoding the ID Token: {0}")]
  DecodeError(#[from] jsonwebtoken::errors::Error),
}

impl TryFrom<GoogleToken> for GoogleClaims {
  type Error = ClaimsError;

  fn try_from(token: GoogleToken) -> Result<Self, Self::Error> {
    let id_token = token.id_token.ok_or(ClaimsError::MissingIdToken)?;

    let token = jsonwebtoken::dangerous_unsafe_decode::<GoogleClaims>(&id_token)?;
    log::info!("Decoded claims: {:?}", token);

    Ok(token.claims)
  }
}

#[async_trait]
impl ProviderCompleteAuth for Provider {
  async fn complete_auth(&self, params: HashMap<String, String>) -> Result<(), ()> {
    let client = reqwest::Client::new();

    let params = [
      ("grant_type", "authorization_code"),
      ("client_id", self.settings.client_id.as_ref()),
      ("client_secret", self.settings.client_secret.as_ref()),
      ("redirect_uri", self.settings.redirect_url.as_ref()),
      ("code", params.get("code").unwrap().as_ref()),
    ];
    log::debug!(
      "Making request to URL {} with params {:?}",
      self.settings.token_url,
      params
    );

    let response = client
      .post(&self.settings.token_url)
      .form(&params)
      .send()
      .await
      .unwrap();
    log::debug!("Response from Google: {:#?}", response);

    if response.status().as_u16() != 200 {
      let body = response.text().await.unwrap();
      log::warn!("Unsuccessful response received from Google: {}", body);
      todo!()
    }

    let body: GoogleToken = response.json().await.unwrap();
    let claims: GoogleClaims = body.try_into().unwrap();
    log::debug!("User Claims from Google: {:?}", claims);

    Ok(())
  }
}
