//! Example programs demonstrating userspace functionality
//!
//! This module contains various examples showing how to use
//! the userspace library's features.

#[path = "examples/image_example.rs"]
pub mod image_example;

// Function to run all examples
pub fn run_examples() {
    image_example::run_image_example();
}

// Add more example modules here as they are created
