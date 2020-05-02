use std::collections::HashMap;

/// The health of a single component
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
    Healthy,
    Unhealthy(String),
}

impl ComponentHealth {
    /// Determine if the component is healthy or not
    pub fn is_healthy(&self) -> bool {
        match self {
            ComponentHealth::Healthy => true,
            ComponentHealth::Unhealthy(_) => false,
        }
    }
}

/// The health of the entire system
#[derive(Debug, PartialEq)]
pub struct SystemHealth {
    pub components: HashMap<String, ComponentHealth>,
}

impl SystemHealth {
    /// Construct the health of the system
    pub fn new(components: HashMap<String, ComponentHealth>) -> Self {
        Self { components }
    }

    /// Determine if the system is healthy or not
    pub fn is_healthy(&self) -> bool {
        self.components
            .iter()
            .all(|(_, component)| component.is_healthy())
    }
}
