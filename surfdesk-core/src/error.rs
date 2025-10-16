//! # Error Handling Module
//!
//! This module provides comprehensive error handling for the SurfDesk application.
//! It defines custom error types, result aliases, and error conversion utilities
//! to ensure consistent error handling across all platforms.

use thiserror::Error;

/// Type alias for Result with SurfDeskError
pub type Result<T> = std::result::Result<T, SurfDeskError>;

/// Comprehensive error type for SurfDesk
#[derive(Error, Debug, Clone)]
pub enum SurfDeskError {
    /// Invalid Solana public key error
    #[error("Invalid Solana pubkey: {0}")]
    InvalidPubkey(String),

    /// Configuration related errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Database related errors
    #[error("Database error: {0}")]
    Database(String),

    /// Database connection errors
    #[error("Database connection error: {0}")]
    DatabaseConnection(String),

    /// Solana RPC errors
    #[error("Solana RPC error: {0}")]
    SolanaRpc(String),

    /// Solana SDK errors
    #[error("Solana SDK error: {0}")]
    SolanaSdk(String),

    /// Anchor program errors
    #[error("Anchor error: {0}")]
    Anchor(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// TOML serialization/deserialization errors
    #[error("TOML serialization error: {0}")]
    TomlSerialization(#[from] toml::ser::Error),

    /// TOML deserialization errors
    #[error("TOML deserialization error: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(String),

    /// Network errors
    #[error("Network error: {0}")]
    Network(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Platform-specific errors
    #[error("Platform error: {0}")]
    Platform(String),

    /// Component errors
    #[error("Component error: {0}")]
    Component(String),

    /// Service errors
    #[error("Service error: {0}")]
    Service(String),

    /// State management errors
    #[error("State error: {0}")]
    State(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Permission errors
    #[error("Permission error: {0}")]
    Permission(String),

    /// Timeout errors
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Not found errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Already exists errors
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    /// Unsupported operation errors
    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    /// Generic errors
    #[error("Internal error: {0}")]
    Internal(String),

    /// Anyhow errors (for compatibility)
    #[error("Error: {0}")]
    Anyhow(String),

    /// UUID parsing errors
    #[error("Invalid UUID: {0}")]
    Uuid(#[from] uuid::Error),

    /// Chrono time parsing errors
    #[error("Time parsing error: {0}")]
    Chrono(#[from] chrono::ParseError),

    /// Custom errors with context
    #[error("{message}: {source}")]
    WithContext {
        message: String,
        source: Box<SurfDeskError>,
    },
}

impl SurfDeskError {
    /// Create a new internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a new network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network(message.into())
    }

    /// Create a new not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(resource.into())
    }

    /// Create a new platform error
    pub fn platform(message: impl Into<String>) -> Self {
        Self::Platform(message.into())
    }

    /// Create a new component error
    pub fn component(message: impl Into<String>) -> Self {
        Self::Component(message.into())
    }

    /// Create a new service error
    pub fn service(message: impl Into<String>) -> Self {
        Self::Service(message.into())
    }

    /// Create a new state error
    pub fn state(message: impl Into<String>) -> Self {
        Self::State(message.into())
    }

    /// Create a new Solana SDK error
    pub fn solana_sdk(message: impl Into<String>) -> Self {
        Self::SolanaSdk(message.into())
    }

    /// Create a new Anchor error
    pub fn anchor(message: impl Into<String>) -> Self {
        Self::Anchor(message.into())
    }

    /// Create a new authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication(message.into())
    }

    /// Create a new permission error
    pub fn permission(message: impl Into<String>) -> Self {
        Self::Permission(message.into())
    }

    /// Create a new timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::Timeout(message.into())
    }

    /// Create a new already exists error
    pub fn already_exists(resource: impl Into<String>) -> Self {
        Self::AlreadyExists(resource.into())
    }

    /// Create a new unsupported error
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::Unsupported(message.into())
    }

    /// Create a new config error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    /// Create a new database error
    pub fn database(message: impl Into<String>) -> Self {
        Self::Database(message.into())
    }

    /// Add context to an existing error
    pub fn with_context(self, message: impl Into<String>) -> Self {
        Self::WithContext {
            message: message.into(),
            source: Box::new(self),
        }
    }

    /// Check if this is a network-related error
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::Network(_) | Self::SolanaRpc(_))
    }

    /// Check if this is a configuration error
    pub fn is_config_error(&self) -> bool {
        matches!(self, Self::Config(_))
    }

    /// Check if this is a database error
    pub fn is_database_error(&self) -> bool {
        matches!(self, Self::Database(_) | Self::DatabaseConnection(_))
    }

    /// Check if this is a validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Check if this is a permission error
    pub fn is_permission_error(&self) -> bool {
        matches!(self, Self::Permission(_) | Self::Authentication(_))
    }

    /// Check if this is a timeout error
    pub fn is_timeout_error(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }

    /// Check if this is a not found error
    pub fn is_not_found_error(&self) -> bool {
        matches!(self, Self::NotFound(_))
    }

    /// Get the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            Self::InvalidPubkey(_) => "Solona",
            Self::Config(_) => "config",
            Self::Database(_) | Self::DatabaseConnection(_) => "database",
            Self::SolanaRpc(_) | Self::SolanaSdk(_) | Self::Anchor(_) => "solana",
            Self::Serialization(_) => "serialization",
            Self::TomlSerialization(_) | Self::TomlDeserialization(_) => "toml",
            Self::Io(_) => "io",
            Self::Network(_) => "network",
            Self::Validation(_) => "validation",
            Self::Platform(_) => "platform",
            Self::Component(_) => "component",
            Self::Service(_) => "service",
            Self::State(_) => "state",
            Self::Authentication(_) | Self::Permission(_) => "auth",
            Self::Timeout(_) => "timeout",
            Self::NotFound(_) => "not_found",
            Self::AlreadyExists(_) => "already_exists",
            Self::Unsupported(_) => "unsupported",
            Self::Internal(_) => "internal",
            Self::Anyhow(_) => "anyhow",
            Self::Uuid(_) => "uuid",
            Self::Chrono(_) => "chrono",
            Self::WithContext { .. } => "with_context",
        }
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Self::Network(_) => {
                "Network connection failed. Please check your internet connection.".to_string()
            }
            Self::Config(_) => "Configuration error. Please check your settings.".to_string(),
            Self::Database(_) | Self::DatabaseConnection(_) => {
                "Database error. Please try again.".to_string()
            }
            Self::SolanaRpc(_) => {
                "Failed to connect to Solana network. Please check your RPC settings.".to_string()
            }
            Self::Validation(msg) => format!("Validation error: {}", msg),
            Self::Permission(_) => {
                "Permission denied. Please check your access rights.".to_string()
            }
            Self::Authentication(_) => {
                "Authentication failed. Please check your credentials.".to_string()
            }
            Self::Timeout(_) => "Operation timed out. Please try again.".to_string(),
            Self::NotFound(msg) => format!("Resource not found: {}", msg),
            Self::AlreadyExists(msg) => format!("Resource already exists: {}", msg),
            Self::Unsupported(msg) => format!("Unsupported operation: {}", msg),
            Self::Io(_) => {
                "File system error. Please check your permissions and disk space.".to_string()
            }
            Self::Internal(msg) => format!("Internal error: {}. Please report this issue.", msg),
            _ => self.to_string(),
        }
    }
}

// Conversion from external error types
impl From<config::ConfigError> for SurfDeskError {
    fn from(err: config::ConfigError) -> Self {
        Self::Config(err.to_string())
    }
}

impl From<diesel::result::Error> for SurfDeskError {
    fn from(err: diesel::result::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<diesel::ConnectionError> for SurfDeskError {
    fn from(err: diesel::ConnectionError) -> Self {
        Self::DatabaseConnection(err.to_string())
    }
}

impl From<solana_client::client_error::ClientError> for SurfDeskError {
    fn from(err: solana_client::client_error::ClientError) -> Self {
        Self::SolanaRpc(err.to_string())
    }
}

impl From<serde_json::Error> for SurfDeskError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<std::io::Error> for SurfDeskError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<anyhow::Error> for SurfDeskError {
    fn from(err: anyhow::Error) -> Self {
        Self::Anyhow(err.to_string())
    }
}

// Conversion from Solana SDK errors
impl From<solana_sdk::program_error::ProgramError> for SurfDeskError {
    fn from(err: solana_sdk::program_error::ProgramError) -> Self {
        Self::SolanaSdk(format!("Program error: {:?}", err))
    }
}

// Conversion from Anchor errors
impl From<anchor_lang::error::Error> for SurfDeskError {
    fn from(err: anchor_lang::error::Error) -> Self {
        Self::Anchor(format!("Anchor error: {:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = SurfDeskError::internal("Test internal error");
        assert!(matches!(err, SurfDeskError::Internal(_)));
        assert_eq!(err.category(), "internal");
    }

    #[test]
    fn test_error_context() {
        let base_err = SurfDeskError::network("Connection failed");
        let contextual_err = base_err.with_context("While connecting to RPC");

        match contextual_err {
            SurfDeskError::WithContext { message, source } => {
                assert_eq!(message, "While connecting to RPC");
                assert!(matches!(*source, SurfDeskError::Network(_)));
            }
            _ => panic!("Expected WithContext error"),
        }
    }

    #[test]
    fn test_error_categories() {
        let network_err = SurfDeskError::network("test");
        assert_eq!(network_err.category(), "network");
        assert!(network_err.is_network_error());

        let config_err = SurfDeskError::from(config::ConfigError::NotFound("test".to_string()));
        assert_eq!(config_err.category(), "config");
        assert!(config_err.is_config_error());
    }

    #[test]
    fn test_user_messages() {
        let network_err = SurfDeskError::network("test");
        let user_msg = network_err.user_message();
        assert!(user_msg.contains("Network connection failed"));

        let validation_err = SurfDeskError::validation("Invalid input");
        let user_msg = validation_err.user_message();
        assert!(user_msg.contains("Validation error"));
        assert!(user_msg.contains("Invalid input"));
    }

    #[test]
    fn test_error_conversions() {
        // Test serde_json conversion
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let surfdesk_err = SurfDeskError::from(json_err);
        assert!(matches!(surfdesk_err, SurfDeskError::Serialization(_)));

        // Test UUID conversion
        let uuid_err = uuid::Uuid::parse_str("invalid-uuid").unwrap_err();
        let surfdesk_err = SurfDeskError::from(uuid_err);
        assert!(matches!(surfdesk_err, SurfDeskError::Uuid(_)));
    }
}
