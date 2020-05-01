use super::{
    claims::{GoogleClaims, GoogleToken},
    Provider,
};
use crate::authentication::repository::{CompleteAuthError, CompletedAuth, ProviderCompleteAuth};
use async_trait::async_trait;
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
        log::debug!("Response Body from Google: {:?}", body);
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
    use serde_json::{json, Value};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn run_test(
        status_code: u16,
        response: Value,
    ) -> Result<CompletedAuth, CompleteAuthError> {
        let _ = env_logger::try_init();
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/oauth2/v4/token"))
            .respond_with(ResponseTemplate::new(status_code).set_body_json(response))
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

        sut.complete_auth(input).await
    }

    #[actix_rt::test]
    async fn test_unsuccessfully() {
        let result = run_test(
            400,
            json!(
              {
                "error": "invalid_grant",
                "error_description": "Malformed auth code."
              }
            ),
        )
        .await
        .unwrap_err();

        assert_that!(
            &result,
            eq(CompleteAuthError::AuthenticationError(
                "Unsuccessful response received from Google".to_owned()
            ))
        );
    }

    #[actix_rt::test]
    async fn test_missing_id_token() {
        let result = run_test(
            200,
            json!(
              {
                "access_token": "AccessToken",
                "token_type": "Bearer",
                "expires_in": 3600
              }
            ),
        )
        .await
        .unwrap_err();

        assert_that!(
            &result,
            eq(CompleteAuthError::AuthenticationError(
                "ID Token was not present in response".to_owned()
            ))
        );
    }

    #[actix_rt::test]
    async fn test_missing_invalid_token() {
        let result = run_test(
            200,
            json!(
              {
                "id_token": "abc.def.ghi",
                "access_token": "AccessToken",
                "token_type": "Bearer",
                "expires_in": 3600
              }
            ),
        )
        .await
        .unwrap_err();

        assert_that!(
            &result,
            eq(CompleteAuthError::AuthenticationError(
                "Receive malformed ID Token from Google".to_owned()
            ))
        );
    }

    #[actix_rt::test]
    async fn test_id_token_missing_subject() {
        let result = run_test(
            200,
            json!(
              {
                // Generated by jwt.io
                "id_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6InNvbWVVc2VyQGV4YW1wbGUuY29tIiwibmFtZSI6IlRlc3QgVXNlciIsInBpY3R1cmUiOiJodHRwczovL2V4YW1wbGUuY29tL3NvbWVVc2VySW1hZ2UifQ.qjIDDgI46LIxkWgESRJi55iiEHk2f4nChYUjpbFh9xI",
                "access_token": "AccessToken",
                "token_type": "Bearer",
                "expires_in": 3600
              }
            ),
        )
        .await
        .unwrap_err();

        assert_that!(
            &result,
            eq(CompleteAuthError::AuthenticationError(
                "Receive malformed ID Token from Google".to_owned()
            ))
        );
    }

    #[actix_rt::test]
    async fn test_success() {
        let result = run_test(
            200,
            json!(
              {
                // Generated by jwt.io
                "id_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJzb21lVXNlcklkIiwiZW1haWwiOiJzb21lVXNlckBleGFtcGxlLmNvbSIsIm5hbWUiOiJUZXN0IFVzZXIiLCJwaWN0dXJlIjoiaHR0cHM6Ly9leGFtcGxlLmNvbS9zb21lVXNlckltYWdlIn0.p4Q9Ep1hNMHROvN-Ha6bo9txfXk794yz6b52PloAi0g",
                "access_token": "AccessToken",
                "token_type": "Bearer",
                "expires_in": 3600
              }
            ),
        )
        .await
        .unwrap();

        assert_that!(
            &result,
            eq(CompletedAuth {
                external_id: "someUserId".to_owned(),
                display_name: "someUser@example.com".to_owned(),
                name: "Test User".to_owned(),
                avatar_url: Some("https://example.com/someUserImage".to_owned())
            })
        );
    }
}
