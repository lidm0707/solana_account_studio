//! Common test utilities and helpers for SurfDesk testing
//!
//! This module provides shared testing utilities, mock data, and helper functions
//! used across unit and integration tests for the SurfDesk application.

pub mod test_helpers;
pub mod mock_data;

// Re-export commonly used items
pub use test_helpers::*;
pub use mock_data::*;

/// Test configuration constants
pub mod constants {
    /// Default test timeout in milliseconds
    pub const DEFAULT_TIMEOUT: u64 = 5000;

    /// Test account public key
    pub const TEST_PUBKEY: &str = "11111111111111111111111111111112";

    /// Test validator URL
    pub const TEST_VALIDATOR_URL: &str = "http://localhost:8899";

    /// Test program ID
    pub const TEST_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
}

/// Test result type for better error handling
pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Common setup for component tests
pub fn setup_component_test() -> TestResult<()> {
    // Initialize logging for tests
    #[cfg(test)]
    env_logger::try_init().ok_or_else(|_| "Failed to init logger")?;

    Ok(())
}

/// Assert that rendered content contains expected text
pub fn assert_contains(rendered: &str, expected: &str) {
    assert!(
        rendered.contains(expected),
        "Expected rendered content to contain '{}'\nActual content:\n{}",
        expected,
        rendered
    );
}

/// Assert that rendered content does not contain unexpected text
pub fn assert_not_contains(rendered: &str, unexpected: &str) {
    assert!(
        !rendered.contains(unexpected),
        "Expected rendered content NOT to contain '{}'\nActual content:\n{}",
        unexpected,
        rendered
    );
}

/// Assert CSS class presence in rendered content
pub fn assert_has_class(rendered: &str, class: &str) {
    assert_contains(rendered, &format!("class=\"{}\"", class));
}
