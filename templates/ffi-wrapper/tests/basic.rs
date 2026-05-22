// Basic integration test for the FFI wrapper.
// When instantiated, {{crate_name}} is replaced with the actual crate name.

use {{crate_name}}::*;

#[test]
fn bindings_are_available() {
    // Verify the FFI bindings can be called
    // This test is intentionally minimal — real FFI tests
    // require a C library to be linked, which is project-specific.
    assert!(true);
}
