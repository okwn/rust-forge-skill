//! Basic usage example for the library.
//!
//! Run with: cargo run --example basic

use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Import the prelude for convenience
    use mylib::prelude::*;

    // Create a service
    let mut service = Service::new();

    // Create some models
    let model1 = Model::new("first-item".to_string(), 42)?;
    let model2 = Model::with_description("second-item".to_string(), 100, "A sample model")?;

    println!("Created models:");
    println!("  1. {} (value: {})", model1.name(), model1.value());
    println!(
        "  2. {} (value: {}) - {}",
        model2.name(),
        model2.value(),
        model2.description().unwrap_or("no description")
    );

    // Process the models
    let results = service.process_batch(vec![model1, model2])?;

    println!("\nProcessed {} models successfully", results.len());
    println!("Service processed {} items total", service.processed_count());

    // Print stats
    let stats = service.stats();
    println!("\nService stats: {:?}", stats);

    Ok(())
}