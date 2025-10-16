//! Integration test entry point for SurfDesk
//!
//! This module serves as the entry point for all integration tests in the SurfDesk project.
//! Integration tests focus on testing how components, services, and modules work together
//! in cross-platform scenarios.

// Import all integration test modules
mod app_shell_tests;
mod cross_platform_tests;
mod navigation_tests;
mod surfpool_integration;

// Re-export common test utilities
pub use crate::common::*;

/// Test configuration constants for integration tests
pub mod test_config {
    /// Default integration test timeout in milliseconds
    pub const DEFAULT_TIMEOUT: u64 = 10000;

    /// Test account public key for integration tests
    pub const TEST_PUBKEY: &str = "11111111111111111111111111111112";

    /// Test validator URL for integration tests
    pub const TEST_VALIDATOR_URL: &str = "http://localhost:8899";

    /// Test program ID for integration tests
    pub const TEST_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

    /// Test environment configuration
    pub const TEST_ENVIRONMENT: &str = "test";
}

/// Setup function for integration tests
pub fn setup_integration_tests() {
    // Initialize logging for integration tests
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();
}

/// Clean up function for integration tests
pub fn cleanup_integration_tests() {
    // Clean up any test artifacts, temporary files, etc.
    // This is called after integration tests complete
}

#[cfg(test)]
mod test_helpers {
    use super::*;

    #[test]
    fn test_integration_test_configuration() {
        assert!(!test_config::TEST_PUBKEY.is_empty());
        assert!(!test_config::TEST_VALIDATOR_URL.is_empty());
        assert!(!test_config::TEST_PROGRAM_ID.is_empty());
        assert!(test_config::DEFAULT_TIMEOUT > 0);
        assert!(!test_config::TEST_ENVIRONMENT.is_empty());
    }

    #[test]
    fn test_setup_integration_tests() {
        // This should not panic
        setup_integration_tests();

        // Clean up after test
        cleanup_integration_tests();
    }

    #[test]
    fn test_cleanup_integration_tests() {
        // This should not panic
        cleanup_integration_tests();
    }
}
