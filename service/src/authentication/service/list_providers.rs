use super::AuthenticationService;
#[cfg(test)]
use super::_FauxOriginal_AuthenticationService;
use crate::authentication::{ListProviders, ProviderName};

#[cfg_attr(test, faux::methods)]
impl ListProviders for AuthenticationService {
    /// Get a list of all known authentication providers
    fn list_providers(&self) -> Vec<&ProviderName> {
        self.registry.list_providers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::repository::Registry;
    use crate::players::PlayerService;
    use galvanic_assert::{assert_that, matchers::collection::*};

    #[test]
    fn test_list_providers() {
        let google: ProviderName = "google".parse().unwrap();
        let twitter: ProviderName = "twitter".parse().unwrap();

        let mut registry = Registry::faux();
        let player_service = PlayerService::faux();

        unsafe {
            faux::when!(registry.list_providers).then(|_| vec![&google, &twitter]);
        }

        let sut = AuthenticationService::new(registry, player_service);

        let result = sut.list_providers();

        assert_that!(&result, contains_in_any_order(vec![&google, &twitter]));
    }
}
