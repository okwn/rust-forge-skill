//! A production Rust library with proper error handling and feature flags.
//!
//! # Semver Policy
//!
//! This crate follows semantic versioning:
//! - **PATCH** (0.1.x): Bug fixes, no API changes
//! - **MINOR** (0.x.0): New features, backward compatible
//! - **MAJOR** (x.0.0): Breaking changes
//!
//! # Public API
//!
//! The public API consists of:
//! - [`LibraryError`] - all library errors
//! - [`Model`] - the main data type
//! - [`Service`] - main service for business logic
//!
//! All public items are documented and include usage examples.
//!
//! # Error Design
//!
//! This library uses `thiserror` for error handling:
//! - [`LibraryError`] is the main error enum
//! - Errors are non-exhaustive to allow future extension
//! - Each error variant has a clear, descriptive message
//! - No `anyhow` in library code - all errors are explicit
//!
//! # Feature Flags
//!
//! - `serde` - Enable serialization support (optional)
//!
//! # Example
//!
//! ```
//! use {{crate_name}}::{Service, Model, LibraryError};
//!
//! fn main() -> Result<(), LibraryError> {
//!     let model = Model::new("example".into(), 42)?;
//!     let mut service = Service::new();
//!     let result = service.process(model)?;
//!     println!("Processed: {}", result.name());
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod model;
pub mod service;

// Re-exports for public API
pub use error::LibraryError;
pub use model::Model;
pub use service::Service;

/// Prelude module for convenient imports.
///
/// # Example
///
/// ```
/// use {{crate_name}}::prelude::*;
/// ```
pub mod prelude {
    pub use crate::error::{LibraryError, ValidationError};
    pub use crate::model::Model;
    pub use crate::service::Service;
}