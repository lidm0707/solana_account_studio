//! Surfdesk Core Library
//!
//! This is the core library for the Surfdesk no-code Solana development platform.
//! It provides the main application logic, components, and services for building
//! and managing Solana programs through a visual interface.

use dioxus::prelude::*;

/// Simple test component to debug RSX issues
#[component]
fn TestComponent() -> Element {
    rsx! {
        div { "Hello, World!" }
    }
}

// Modules for the Surfdesk application
// pub mod components;  // Temporarily disabled due to compilation errors
pub mod models;
pub mod pages;
pub mod routes;
// pub mod services;    // Temporarily disabled due to compilation errors
// pub mod utils;       // Temporarily disabled due to compilation errors

// Re-export main types for easier access
// pub use components::*;
pub use pages::*;
pub use routes::*;
// pub use services::*;
// pub use utils::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_library_compiles() {
        // Basic compilation test - the fact this compiles is the test
    }
}
