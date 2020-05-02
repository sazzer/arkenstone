use super::Provider;
use crate::players::ProviderName;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of the providers that we can work with
#[derive(Clone)]
pub struct Registry {
    providers: HashMap<ProviderName, Arc<dyn Provider>>,
}

impl Registry {
    /// Build a new registry
    pub fn new(providers: HashMap<ProviderName, Arc<dyn Provider>>) -> Self {
        Self { providers }
    }

    /// Get the list of providers from the registry
    pub fn list_providers(&self) -> Vec<&ProviderName> {
        self.providers.iter().map(|(key, _)| key).collect()
    }

    /// Attempt to get the requested provider from the registry
    pub fn find_provider(&self, provider: &ProviderName) -> Option<&Arc<dyn Provider>> {
        self.providers.get(provider)
    }
}

/// Builder to use to build the provider registry
#[derive(Default)]
pub struct RegistryBuilder {
    providers: HashMap<ProviderName, Arc<dyn Provider>>,
}

impl RegistryBuilder {
    /// Add a new Provider to the builder
    pub fn with_provider<S>(&mut self, name: S, provider: Arc<dyn Provider>) -> &mut Self
    where
        S: Into<String>,
    {
        self.providers.insert(ProviderName::new(name), provider);

        self
    }

    /// Actually build the registry from what we've so far registered
    pub fn build(&self) -> Registry {
        Registry::new(self.providers.clone())
    }
}
