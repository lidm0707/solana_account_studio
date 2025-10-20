//! Components module
//!
//! This module contains all the UI components for the Surfdesk application.
//! Components are organized by functionality and follow Dioxus patterns.

// Temporarily disable problematic components
// pub mod common;
// pub mod forms;
// pub mod layout;
// pub mod navigation;
pub mod program_builder;

// Re-export commonly used components
// pub use common::*;
// pub use forms::*;
// pub use layout::*;
// pub use navigation::*;
pub use program_builder::*;
