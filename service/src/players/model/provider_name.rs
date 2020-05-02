use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// New Type to represent the name of an authentication provider
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ProviderName(String);

impl ProviderName {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

impl From<ProviderName> for Cow<'static, str> {
    fn from(provider_name: ProviderName) -> Cow<'static, str> {
        provider_name.0.into()
    }
}
