// Integration tests for {{crate_name}} library.
// These tests run against the public API exposed by lib.rs.
// When instantiated, {{crate_name}} is replaced with the actual crate name.

use {{crate_name}}::{Model, Service, LibraryError};
use std::error::Error;

#[test]
fn service_returns_expected_value() -> Result<(), Box<dyn Error>> {
    let service = Service::new();
    let result = service.process(Model::default())?;
    assert_eq!(result.value, "processed");
    Ok(())
}

#[test]
fn error_on_empty_input() {
    let service = Service::new();
    let result = service.process(Model { name: String::new(), value: String::new() });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LibraryError::ValidationError(_)));
}

#[test]
fn model_display() {
    let model = Model { name: "test".into(), value: "value".into() };
    let display = format!("{}", model);
    assert!(display.contains("test"));
}

#[test]
fn error_display() {
    let err = LibraryError::NotFound("resource".into());
    let display = format!("{}", err);
    assert!(display.contains("resource"));
}
