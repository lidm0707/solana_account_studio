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
pub mod components;
pub mod models;
pub mod pages;
pub mod routes;
pub mod services;
