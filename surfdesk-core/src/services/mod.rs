//! Services module for Surfdesk
//!
//! This module contains all the service layer components that handle
//! business logic, external integrations, and data management.
//! Services are organized by functionality and follow clean architecture principles.

pub mod surfpool;
pub mod solana_rpc;
pub mod account;
pub mod program;

// Re-export commonly used services
pub use surfpool::*;
pub use solana_rpc::*;
pub use account::*;
pub use program::*;

/// Common error types for all services
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(String),
}

/// Result type for service operations
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Common trait for all services
pub trait Service {
    /// Initialize the service
    fn initialize(&mut self) -> ServiceResult<()>;

    /// Check if the service is healthy
    fn health_check(&self) -> ServiceResult<bool>;

    /// Shutdown the service
    fn shutdown(&mut self) -> ServiceResult<()>;
}

/// Configuration trait for configurable services
pub trait Configurable {
    type Config;

    /// Configure the service
    fn configure(&mut self, config: Self::Config) -> ServiceResult<()>;

    /// Get current configuration
    fn get_config(&self) -> &Self::Config;
}

/// Async service trait for services that perform async operations
#[async_trait::async_trait]
pub trait AsyncService: Service {
    /// Async initialization
    async fn initialize_async(&mut self) -> ServiceResult<()>;

    /// Async health check
    async fn health_check_async(&self) -> ServiceResult<bool>;

    /// Async shutdown
    async fn shutdown_async(&mut self) -> ServiceResult<()>;
}
