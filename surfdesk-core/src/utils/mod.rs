//! Utility modules for Surfdesk
//!
//! This module contains various utility functions and helpers used throughout
//! the application, including formatting helpers, validation functions,
//! cryptographic utilities, and other common operations.

pub mod crypto;
pub mod format;
pub mod validation;

// Re-export commonly used utilities
pub use crypto::*;
pub use format::*;
pub use validation::*;
