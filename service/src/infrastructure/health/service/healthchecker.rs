use crate::infrastructure::health::{Component, ComponentHealth, SystemHealth};
use std::collections::HashMap;
use std::sync::Arc;

/// The actual service layer that can check the health of the system
#[derive(Clone)]
pub struct HealthcheckerService {
    components: HashMap<String, Arc<dyn Component>>,
}

impl HealthcheckerService {
    /// Create a new instance of the health checker service
    pub fn new(components: HashMap<String, Arc<dyn Component>>) -> Self {
        Self { components }
    }

    /// Actually check the health of the system
    pub async fn check_health(&self) -> SystemHealth {
        let mut results = HashMap::new();

        for (name, component) in &self.components {
            let result = component.check_health().await;

            results.insert(
                name.clone(),
                match result {
                    Ok(_) => ComponentHealth::Healthy,
                    Err(e) => ComponentHealth::Unhealthy(format!("{}", e)),
                },
            );
        }

        SystemHealth::new(results)
    }
}

/// Builder for the service layer that can check the health of the system
#[derive(Default)]
pub struct HealthcheckerServiceBuilder {
    components: HashMap<String, Arc<dyn Component>>,
}

impl HealthcheckerServiceBuilder {
    /// Add a new component to the health checker that we are building
    pub fn with_component<S>(&mut self, name: S, component: Arc<dyn Component>) -> &mut Self
    where
        S: Into<String>,
    {
        self.components.insert(name.into(), component);

        self
    }

    /// Actually build the health checker
    pub fn build(&self) -> HealthcheckerService {
        HealthcheckerService::new(self.components.clone())
    }
}
