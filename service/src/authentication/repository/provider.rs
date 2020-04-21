/// Base trait for starting authentication with a provider
pub trait ProviderStartAuth {}

/// Base trait for completing authentication with a provider
pub trait ProviderCompleteAuth {}

/// Trait that all Providers will implement
pub trait Provider: ProviderStartAuth + ProviderCompleteAuth {}
