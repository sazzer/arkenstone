use super::{
  claims::{GoogleClaims, GoogleToken},
  Provider,
};
use crate::authentication::repository::{CompleteAuthError, CompletedAuth, ProviderCompleteAuth};
use async_trait::async_trait;
use reqwest;
use std::collections::HashMap;
use std::convert::TryInto;

#[async_trait]
impl ProviderCompleteAuth for Provider {
  async fn complete_auth(
    &self,
    params: HashMap<String, String>,
  ) -> Result<CompletedAuth, CompleteAuthError> {
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
      .map_err(|e| CompleteAuthError::AuthenticationError(e.to_string()))?;
    log::debug!("Response from Google: {:#?}", response);

    if response.status().as_u16() != 200 {
      let body = response.text().await.unwrap();
      log::warn!("Unsuccessful response received from Google: {}", body);
      return Err(CompleteAuthError::AuthenticationError(
        "Unsuccessful response received from Google".to_owned(),
      ));
    }

    let body: GoogleToken = response
      .json()
      .await
      .map_err(|e| CompleteAuthError::AuthenticationError(e.to_string()))?;
    let claims: GoogleClaims = body.try_into()?;
    log::debug!("User Claims from Google: {:?}", claims);

    Ok(claims.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::authentication::repository::google::Settings;
  use galvanic_assert::{assert_that, matchers::*};
  use serde_json::json;
  use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
  };

  #[actix_rt::test]
  async fn test_complete_unsuccessfully() {
    let _ = env_logger::try_init();
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path("/oauth2/v4/token"))
      .respond_with(ResponseTemplate::new(400).set_body_json(json!(
        {
          "error": "invalid_grant",
          "error_description": "Malformed auth code."
        }
      )))
      .mount(&mock_server)
      .await;

    let sut = Provider::new(Settings {
      client_id: "someClientId".to_owned(),
      client_secret: "someClientSecret".to_owned(),
      redirect_url: "http://localhost:8080/authentication/google/complete".to_owned(),
      auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_owned(),
      token_url: format!("{}/oauth2/v4/token", &mock_server.uri()),
    });

    let mut input = HashMap::new();
    input.insert("code".to_owned(), "someAuthCode".to_owned());

    let result = sut.complete_auth(input).await.unwrap_err();
    assert_that!(
      &result,
      eq(CompleteAuthError::AuthenticationError(
        "Unsuccessful response received from Google".to_owned()
      ))
    );
  }
}
