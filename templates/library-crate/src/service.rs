//! Service layer for business logic.
//!
//! This module provides the main [`Service`] type which encapsulates
//! the core business logic of the library.

use crate::error::LibraryError;
use crate::model::Model;

/// A service that processes models.
///
/// # Example
///
/// ```
/// use {{crate_name}}::{Service, Model};
///
/// let mut service = Service::new();
/// let model = Model::new("test".to_string(), 42).unwrap();
/// let result = service.process(model);
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone, Default)]
pub struct Service {
    /// Internal counter for processed items.
    processed_count: usize,
}

impl Service {
    /// Creates a new service instance.
    pub fn new() -> Self {
        Self {
            processed_count: 0,
        }
    }

    /// Creates a service with a specific initial count.
    pub fn with_count(processed_count: usize) -> Self {
        Self { processed_count }
    }

    /// Returns the number of items processed by this service.
    pub fn processed_count(&self) -> usize {
        self.processed_count
    }

    /// Processes a single model.
    ///
    /// This method validates and processes the given model, updating
    /// internal state as appropriate.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryError`] if processing fails.
    ///
    /// # Example
    ///
    /// ```
    /// use {{crate_name}}::{Service, Model};
    ///
    /// let mut service = Service::new();
    /// let model = Model::new("test".to_string(), 42).unwrap();
    /// let result = service.process(model);
    /// assert!(result.is_ok());
    /// ```
    pub fn process(&mut self, model: Model) -> Result<Model, LibraryError> {
        self.processed_count += 1;
        Ok(model)
    }

    /// Processes a batch of models.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryError`] if any model fails processing.
    ///
    /// # Example
    ///
    /// ```
    /// use {{crate_name}}::{Service, Model};
    ///
    /// let mut service = Service::new();
    /// let models = vec![
    ///     Model::new("test1".to_string(), 1).unwrap(),
    ///     Model::new("test2".to_string(), 2).unwrap(),
    /// ];
    /// let results = service.process_batch(models);
    /// assert!(results.is_ok());
    /// assert_eq!(results.unwrap().len(), 2);
    /// ```
    pub fn process_batch(&mut self, models: impl IntoIterator<Item = Model>) -> Result<Vec<Model>, LibraryError> {
        let mut processed = Vec::new();
        for model in models.into_iter() {
            let result = self.process(model)?;
            processed.push(result);
        }
        Ok(processed)
    }

    /// Processes multiple models concurrently-like (sequentially for now).
    ///
    /// This method processes each model and collects successful results,
    /// stopping on the first error.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryError`] if any model fails processing.
    pub fn process_all(&mut self, models: impl IntoIterator<Item = Model>) -> Result<Vec<Model>, LibraryError> {
        self.process_batch(models)
    }

    /// Validates a model without processing it.
    ///
    /// # Example
    ///
    /// ```
    /// use {{crate_name}}::{Service, Model};
    ///
    /// let service = Service::new();
    /// let model = Model::new("test".to_string(), 42).unwrap();
    /// assert!(service.validate(&model).is_ok());
    /// ```
    pub fn validate(&self, _model: &Model) -> Result<(), LibraryError> {
        Ok(())
    }

    /// Resets the service state.
    pub fn reset(&mut self) {
        self.processed_count = 0;
    }

    /// Gets statistics about the service.
    pub fn stats(&self) -> ServiceStats {
        ServiceStats {
            processed_count: self.processed_count,
        }
    }
}

/// Statistics about the service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ServiceStats {
    /// Number of items processed.
    pub processed_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_service_new() {
        let service = Service::new();
        assert_eq!(service.processed_count(), 0);
    }

    #[test]
    fn test_service_process() {
        let mut service = Service::new();
        let model = Model::new("test".to_string(), 42).unwrap();
        let result = service.process(model).unwrap();
        assert_eq!(result.name(), "test");
        assert_eq!(service.processed_count(), 1);
    }

    #[test]
    fn test_service_process_batch() {
        let mut service = Service::new();
        let models = vec![
            Model::new("test1".to_string(), 1).unwrap(),
            Model::new("test2".to_string(), 2).unwrap(),
        ];
        let results = service.process_batch(models).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(service.processed_count(), 2);
    }

    #[test]
    fn test_service_process_batch_empty() {
        let mut service = Service::new();
        let models: Vec<Model<i32>> = vec![];
        let results = service.process_batch(models).unwrap();
        assert_eq!(results.len(), 0);
        assert_eq!(service.processed_count(), 0);
    }

    #[test]
    fn test_service_validate() {
        let service = Service::new();
        let model = Model::new("test".to_string(), 42).unwrap();
        assert!(service.validate(&model).is_ok());
    }

    #[test]
    fn test_service_reset() {
        let mut service = Service::new();
        let model = Model::new("test".to_string(), 42).unwrap();
        service.process(model).unwrap();
        assert_eq!(service.processed_count(), 1);
        service.reset();
        assert_eq!(service.processed_count(), 0);
    }

    #[test]
    fn test_service_stats() {
        let mut service = Service::new();
        let model = Model::new("test".to_string(), 42).unwrap();
        service.process(model).unwrap();
        let stats = service.stats();
        assert_eq!(stats.processed_count, 1);
    }

    #[rstest]
    #[case::single_item(vec!["item1"], 1)]
    #[case::multiple_items(vec!["item1", "item2", "item3"], 3)]
    fn test_process_count(#[case] names: Vec<&str>, #[case] expected_count: usize) {
        let mut service = Service::new();
        for name in names {
            let model = Model::new(name.to_string(), 0).unwrap();
            service.process(model).unwrap();
        }
        assert_eq!(service.processed_count(), expected_count);
    }
}