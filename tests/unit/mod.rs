//! Unit test entry point for SurfDesk
//!
//! This module serves as the entry point for all unit tests in the SurfDesk project.
//! Unit tests focus on testing individual components, services, and modules in isolation.

// Import all unit test modules
mod components;
mod database;
mod services;

// Re-export common test utilities
pub use crate::common::*;

/// Test configuration constants
pub mod test_config {
    /// Default test timeout in milliseconds
    pub const DEFAULT_TIMEOUT: u64 = 5000;

    /// Test account public key
    pub const TEST_PUBKEY: &str = "11111111111111111111111111111112";

    /// Test validator URL
    pub const TEST_VALIDATOR_URL: &str = "http://localhost:8899";

    /// Test program ID
    pub const TEST_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
}

/// Setup function for unit tests
pub fn setup_unit_tests() {
    // Initialize logging for tests
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();
}

#[cfg(test)]
mod test_helpers {
    use super::*;

    #[test]
    fn test_test_configuration() {
        assert!(!test_config::TEST_PUBKEY.is_empty());
        assert!(!test_config::TEST_VALIDATOR_URL.is_empty());
        assert!(!test_config::TEST_PROGRAM_ID.is_empty());
        assert!(test_config::DEFAULT_TIMEOUT > 0);
    }

    #[test]
    fn test_setup_unit_tests() {
        // This should not panic
        setup_unit_tests();
    }
}
