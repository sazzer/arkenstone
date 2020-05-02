#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::module_name_repetitions,
        clippy::module_inception,
        clippy::used_underscore_binding,
        clippy::wildcard_imports,
        clippy::missing_errors_doc
    )
)]

mod authentication;
mod health;
pub(crate) mod service;
