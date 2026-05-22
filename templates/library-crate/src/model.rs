//! Data models for the library.
//!
//! This module defines the core data types used throughout the library.

use crate::error::{LibraryError, ValidationError};

/// The maximum allowed value.
const MAX_VALUE: i64 = 1000;

/// The minimum allowed value.
const MIN_VALUE: i64 = -1000;

/// A model struct representing the main data type.
///
/// # Type Parameters
///
/// * `T` - The value type, must implement `Copy` and `From<i32>`
///
/// # Example
///
/// ```
/// use {{crate_name}}::Model;
///
/// let model = Model::new("test".to_string(), 42).unwrap();
/// assert_eq!(model.name(), "test");
/// assert_eq!(model.value(), 42);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Model<T = i32>
where
    T: Copy + From<i32> + std::fmt::Debug,
{
    /// The name of the model.
    name: String,

    /// The value of the model.
    value: T,

    /// Optional description.
    description: Option<String>,
}

impl<T> Model<T>
where
    T: Copy + From<i32> + std::fmt::Debug,
{
    /// Creates a new model with the given name and value.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryError::Validation`] if the name is empty or contains
    /// invalid characters.
    ///
    /// # Example
    ///
    /// ```
    /// use {{crate_name}}::Model;
    ///
    /// let model = Model::new("test".to_string(), 42).unwrap();
    /// ```
    pub fn new(name: String, value: T) -> Result<Self, LibraryError> {
        ValidationError::validate_name(&name).map_err(LibraryError::from)?;
        Ok(Self {
            name,
            value,
            description: None,
        })
    }

    /// Creates a new model with a description.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryError::Validation`] if the name is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use {{crate_name}}::Model;
    ///
    /// let model = Model::with_description("test".to_string(), 42, "A test model").unwrap();
    /// assert_eq!(model.description(), Some("A test model"));
    /// ```
    pub fn with_description(
        name: String,
        value: T,
        description: impl Into<String>,
    ) -> Result<Self, LibraryError> {
        ValidationError::validate_name(&name).map_err(LibraryError::from)?;
        Ok(Self {
            name,
            value,
            description: Some(description.into()),
        })
    }

    /// Returns the name of the model.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value of the model.
    pub fn value(&self) -> T {
        self.value
    }

    /// Returns the description if set.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the description.
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
    }

    /// Sets the value.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryError::Processing`] if the value is out of range.
    pub fn set_value(&mut self, value: T) -> Result<(), LibraryError>
    where
        T: TryInto<i64>,
        <T as TryInto<i64>>::Error: std::fmt::Debug,
    {
        let value_i64: i64 = value.try_into().unwrap();
        if value_i64 > MAX_VALUE || value_i64 < MIN_VALUE {
            return Err(LibraryError::processing(format!(
                "value must be between {} and {}, got {}",
                MIN_VALUE, MAX_VALUE, value_i64
            )));
        }
        self.value = value;
        Ok(())
    }

    /// Converts the model to a display string.
    pub fn to_display_string(&self) -> String {
        match &self.description {
            Some(desc) => format!("{} ({:?}): {}", self.name, self.value, desc),
            None => format!("{} ({:?}): No description", self.name, self.value),
        }
    }
}

impl Model<i32> {
    /// Creates a model with default value (0).
    ///
    /// # Example
    ///
    /// ```
    /// use {{crate_name}}::Model;
    ///
    /// let model = Model::with_name("test".to_string()).unwrap();
    /// assert_eq!(model.value(), 0);
    /// ```
    pub fn with_name(name: String) -> Result<Self, LibraryError> {
        Self::new(name, i32::from(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_model_creation() {
        let model = Model::new("test".to_string(), 42).unwrap();
        assert_eq!(model.name(), "test");
        assert_eq!(model.value(), 42);
        assert_eq!(model.description(), None);
    }

    #[test]
    fn test_model_with_description() {
        let model = Model::with_description("test".to_string(), 42, "A test").unwrap();
        assert_eq!(model.description(), Some("A test"));
    }

    #[test]
    fn test_model_empty_name() {
        let result = Model::<i32>::new("".to_string(), 42);
        assert!(result.is_err());
    }

    #[test]
    fn test_model_set_value() {
        let mut model = Model::new("test".to_string(), 42).unwrap();
        model.set_value(100).unwrap();
        assert_eq!(model.value(), 100);
    }

    #[test]
    fn test_model_set_value_out_of_range() {
        let mut model = Model::new("test".to_string(), 42).unwrap();
        let result = model.set_value(2000i32);
        assert!(result.is_err());
    }

    #[rstest]
    #[case::valid_alphanumeric("valid_name-123", true)]
    #[case::valid_underscore("valid_name", true)]
    #[case::invalid_special_chars("test@name", false)]
    #[case::invalid_spaces("test name", false)]
    #[case::empty("", false)]
    fn test_name_validation(#[case] name: &str, #[case] expected_valid: bool) {
        let result = ValidationError::validate_name(name).is_ok();
        assert_eq!(result, expected_valid);
    }

    #[test]
    fn test_to_display_string() {
        let model = Model::with_description("test".to_string(), 42, "desc").unwrap();
        assert_eq!(model.to_display_string(), "test (42): desc");
    }

    #[test]
    fn test_model_clone() {
        let model = Model::new("test".to_string(), 42).unwrap();
        let cloned = model.clone();
        assert_eq!(model, cloned);
    }

    #[test]
    fn test_model_eq() {
        let model1 = Model::new("test".to_string(), 42).unwrap();
        let model2 = Model::new("test".to_string(), 42).unwrap();
        assert_eq!(model1, model2);
    }
}