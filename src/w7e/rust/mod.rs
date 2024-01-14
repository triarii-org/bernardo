// all stuff that helps reading Rust projects
mod handler_rust;
pub use handler_rust::{RustHandler, INIT_TIMEOUT};

mod inspector_rust;
pub use inspector_rust::RustLangInspector;

#[cfg(test)]
mod workspace_test;
