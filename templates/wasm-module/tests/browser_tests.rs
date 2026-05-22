use wasm_module::*;

#[wasm_bindgen_test]
fn test_greet() {
    assert_eq!(greet("Test"), "Hello, Test!");
}

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[wasm_bindgen_test]
fn test_point() {
    let p = create_point(3.0, 4.0);
    assert!((p.x() - 3.0).abs() < 1e-10);
    assert!((p.y() - 4.0).abs() < 1e-10);
}