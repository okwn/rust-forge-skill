use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

pub mod utils;

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Greet a name with a friendly message.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Add two integers.
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two floating point numbers.
#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Process a byte array, incrementing each byte by 1.
#[wasm_bindgen]
pub fn process_bytes(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b.wrapping_add(1)).collect()
}

/// A 2D point that can be passed to/from JavaScript.
#[derive(Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Calculate distance to another point.
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[wasm_bindgen]
pub fn create_point(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

#[wasm_bindgen]
pub fn points_distance(p1: &Point, p2: &Point) -> f64 {
    p1.distance_to(p2)
}

/// JSON serialization helper
#[wasm_bindgen]
pub fn serialize_point(point: &Point) -> String {
    serde_json::to_string(point).unwrap_or_default()
}

/// Deserialize from JSON
#[wasm_bindgen]
pub fn deserialize_point(json: &str) -> Option<Point> {
    serde_json::from_str(json).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("World");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert!((multiply(3.0, 4.0) - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_process_bytes() {
        let data = vec![1, 2, 3];
        let result = process_bytes(&data);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let dist = p1.distance_to(&p2);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_serialize_point() {
        let p = Point::new(1.0, 2.0);
        let json = serialize_point(&p);
        assert!(json.contains("1"));
        assert!(json.contains("2"));
    }
}