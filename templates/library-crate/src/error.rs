//! Error types for the library.
//!
//! This module defines all errors that can occur in the library.
//! All errors use `thiserror` for ergonomic error handling.

use thiserror::Error;

/// Maximum allowed name length.
const MAX_NAME_LENGTH: usize = 100;

/// Main library error type.
///
/// This enum is non-exhaustive to allow future error variants to be added
/// without breaking changes.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LibraryError {
    /// Name validation failed.
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),

    /// The name exceeds the maximum allowed length.
    #[error("name too long: max={max}, actual={actual}")]
    NameTooLong { max: usize, actual: usize },

    /// Entity not found.
    #[error("entity not found: {0}")]
    NotFound(String),

    /// Processing failed.
    #[error("processing failed: {reason}")]
    Processing { reason: String },
}

/// Validation-specific errors.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Name cannot be empty.
    #[error("name cannot be empty")]
    EmptyName,

    /// Name contains invalid characters.
    #[error("name contains invalid characters: {0}")]
    InvalidCharacters(String),

    /// Name is too long.
    #[error("name too long: max={max}, actual={actual}")]
    NameTooLong { max: usize, actual: usize },
}

impl LibraryError {
    /// Creates a not found error.
    pub fn not_found(id: impl Into<String>) -> Self {
        LibraryError::NotFound(id.into())
    }

    /// Creates a processing error with a reason.
    pub fn processing(reason: impl Into<String>) -> Self {
        LibraryError::Processing {
            reason: reason.into(),
        }
    }

    /// Creates a name too long error.
    pub fn name_too_long(actual: usize) -> Self {
        LibraryError::NameTooLong {
            max: MAX_NAME_LENGTH,
            actual,
        }
    }
}

impl ValidationError {
    /// Validates a name and returns an error if invalid.
    pub fn validate_name(name: &str) -> Result<(), Self> {
        if name.is_empty() {
            return Err(ValidationError::EmptyName);
        }

        if name.len() > MAX_NAME_LENGTH {
            return Err(ValidationError::NameTooLong {
                max: MAX_NAME_LENGTH,
                actual: name.len(),
            });
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(ValidationError::InvalidCharacters(
                "name must contain only alphanumeric characters, underscores, or hyphens"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_empty_name() {
        let result = ValidationError::validate_name("");
        assert!(matches!(result, Err(ValidationError::EmptyName)));
    }

    #[test]
    fn test_validation_error_valid_name() {
        let result = ValidationError::validate_name("valid_name-123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_error_name_too_long() {
        let long_name = "a".repeat(101);
        let result = ValidationError::validate_name(&long_name);
        assert!(matches!(
            result,
            Err(ValidationError::NameTooLong { max: 100, actual: 101 })
        ));
    }

    #[test]
    fn test_library_error_not_found() {
        let err = LibraryError::not_found("item-123");
        assert!(matches!(err, LibraryError::NotFound(ref s) if s == "item-123"));
    }
}