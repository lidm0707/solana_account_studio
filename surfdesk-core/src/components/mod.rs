//! Components module
//!
//! This module contains all the UI components for the Surfdesk application.
//! Components are organized by functionality and follow Dioxus patterns.

pub mod common;
pub mod forms;
pub mod layout;
pub mod navigation;

// Re-export commonly used components
pub use common::*;
pub use forms::*;
pub use layout::*;
pub use navigation::*;
