use crate::authentication::repository::{CompleteAuthError, CompletedAuth};
use crate::players::ProviderPlayerId;
use serde::Deserialize;
use std::convert::TryFrom;

/// The shape of the Token details retrieved from Google
#[derive(Debug, Deserialize)]
pub struct GoogleToken {
    id_token: Option<String>,
}

/// The shape of the claims inside the ID Token from Google
#[derive(Debug, Deserialize)]
pub struct GoogleClaims {
    sub: ProviderPlayerId,
    email: String,
    name: String,
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

impl From<GoogleClaims> for CompletedAuth {
    fn from(claims: GoogleClaims) -> Self {
        Self {
            external_id: claims.sub,
            display_name: claims.email,
            name: claims.name,
            avatar_url: claims.picture,
        }
    }
}

impl From<ClaimsError> for CompleteAuthError {
    fn from(e: ClaimsError) -> Self {
        match e {
            ClaimsError::MissingIdToken => CompleteAuthError::AuthenticationError(
                "ID Token was not present in response".to_owned(),
            ),
            ClaimsError::DecodeError(_) => CompleteAuthError::AuthenticationError(
                "Receive malformed ID Token from Google".to_owned(),
            ),
        }
    }
}
