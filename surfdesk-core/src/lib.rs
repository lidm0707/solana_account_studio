//! # SurfDesk Core Library
//!
//! This is the core library for SurfDesk - a comprehensive Solana account studio
//! built with Dioxus 0.6+ for cross-platform deployment (desktop, web, and terminal).
//!
//! ## Features
//!
//! - **Cross-platform**: Shared business logic for desktop, web, and terminal
//! - **Solana Integration**: Full Solana blockchain interaction capabilities
//! - **Component Architecture**: Reusable UI components with Dioxus
//! - **Service Layer**: Modular service architecture for blockchain operations
//! - **Database Integration**: SQLite database with Diesel ORM for persistence
//! - **Configuration Management**: Flexible configuration system
//! - **Error Handling**: Comprehensive error handling with proper propagation

#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

pub mod app;
pub mod components;
pub mod database;
pub mod error;
pub mod platform;
pub mod services;
pub mod state;
pub mod types;

// Re-export commonly used items
pub use app::SurfDeskApp;
pub use error::{Result, SurfDeskError};
pub use platform::Platform;
pub use state::AppState;
pub use types::*;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default Solana RPC URL
pub const DEFAULT_SOLANA_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

/// Local Solana validator URL
pub const LOCAL_VALIDATOR_URL: &str = "http://localhost:8899";

/// Solana devnet URL
pub const DEVNET_URL: &str = "https://api.devnet.solana.com";

/// Solana testnet URL
pub const TESTNET_URL: &str = "https://api.testnet.solana.com";

/// Initialize the SurfDesk core library
///
/// This function must be called before using any core functionality.
/// It sets up logging, configuration, and initializes global state.
///
/// # Errors
///
/// Returns an error if initialization fails
///
/// # Examples
///
/// ```rust
/// use surfdesk_core::init_core;
///
/// #[tokio::main]
/// async fn main() -> surfdesk_core::Result<()> {
///     init_core().await?;
///     // Your application code here
///     Ok(())
/// }
/// ```
pub async fn init_core() -> Result<()> {
    // Initialize logger
    #[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
    {
        env_logger::init();
    }

    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Failed to initialize console log");
    }

    log::info!("SurfDesk Core v{} initializing", VERSION);

    // Initialize configuration
    #[cfg(feature = "database")]
    {
        services::config::init_config().await?;
    }

    log::info!("SurfDesk Core initialized successfully");
    Ok(())
}

/// Get the current platform information
///
/// # Returns
///
/// Returns a `Platform` enum representing the current platform
pub fn current_platform() -> Platform {
    #[cfg(target_arch = "wasm32")]
    {
        Platform::Web
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "tui"))]
    {
        Platform::Terminal
    }

    #[cfg(all(not(target_arch = "wasm32"), not(feature = "tui")))]
    {
        Platform::Desktop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_default_urls() {
        assert!(!DEFAULT_SOLANA_RPC_URL.is_empty());
        assert!(!LOCAL_VALIDATOR_URL.is_empty());
        assert!(!DEVNET_URL.is_empty());
        assert!(!TESTNET_URL.is_empty());
    }

    #[tokio::test]
    async fn test_init_core() {
        // This test will initialize the core library
        // In real applications, you should only call init_core once
        let result = init_core().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_current_platform() {
        let platform = current_platform();
        // The platform should be one of the valid variants
        match platform {
            Platform::Desktop | Platform::Web | Platform::Terminal => {}
        }
    }
}
