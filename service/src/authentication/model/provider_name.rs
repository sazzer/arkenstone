use std::str::FromStr;

/// New Type to represent the name of an authentication provider
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProviderName(String);

/// Errors that can occur when parsing a provider name
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ProviderNameParseError {
  #[error("The value must not be blank")]
  Blank,
}

impl FromStr for ProviderName {
  type Err = ProviderNameParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let trimmed_name = s.trim();

    if trimmed_name.len() == 0 {
      Err(ProviderNameParseError::Blank)
    } else {
      Ok(Self(trimmed_name.to_owned()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
  };

  #[test]
  fn test_parse_name() {
    let input = "google";
    let parsed: Result<ProviderName, ProviderNameParseError> = input.parse();

    assert_that!(&parsed, maybe_ok(eq(ProviderName("google".to_owned()))));
  }

  #[test]
  fn test_parse_padded_name() {
    let input = "   google   ";
    let parsed: Result<ProviderName, ProviderNameParseError> = input.parse();

    assert_that!(&parsed, maybe_ok(eq(ProviderName("google".to_owned()))));
  }

  #[test]
  fn test_parse_empty_name() {
    let input = "";
    let parsed: Result<ProviderName, ProviderNameParseError> = input.parse();

    assert_that!(&parsed, maybe_err(eq(ProviderNameParseError::Blank)));
  }

  #[test]
  fn test_parse_blank_name() {
    let input = "   ";
    let parsed: Result<ProviderName, ProviderNameParseError> = input.parse();

    assert_that!(&parsed, maybe_err(eq(ProviderNameParseError::Blank)));
  }
}
