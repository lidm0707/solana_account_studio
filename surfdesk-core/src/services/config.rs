//! # Configuration Service Module
//!
//! This module provides configuration management for the SurfDesk application.
//! It handles loading, saving, and validating configuration across all platforms
//! with platform-specific storage backends.

use crate::error::{Result, SurfDeskError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration service for managing application settings
pub struct ConfigService {
    /// Current configuration
    config: Config,
    /// Configuration file path
    config_path: PathBuf,
    /// Whether configuration has been modified
    dirty: bool,
}

impl ConfigService {
    /// Create a new configuration service
    pub async fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_config(&config_path).await?;

        Ok(Self {
            config,
            config_path,
            dirty: false,
        })
    }

    /// Get the configuration file path
    fn get_config_path() -> Result<PathBuf> {
        let platform = crate::current_platform();

        let path = match platform {
            crate::platform::Platform::Desktop => {
                let mut path = dirs::config_dir()
                    .ok_or_else(|| SurfDeskError::internal("Could not find config directory"))?;
                path.push("surfdesk");
                path.push("config.toml");
                path
            }
            crate::platform::Platform::Web => {
                // Web uses localStorage - return a placeholder path
                PathBuf::from("localstorage://surfdesk-config")
            }
            crate::platform::Platform::Terminal => {
                // Terminal uses home directory
                let mut path = dirs::home_dir()
                    .ok_or_else(|| SurfDeskError::internal("Could not find home directory"))?;
                path.push(".surfdesk");
                path.push("config.toml");
                path
            }
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(path)
    }

    /// Load configuration from file
    async fn load_config(path: &PathBuf) -> Result<Config> {
        let platform = crate::current_platform();

        match platform {
            crate::platform::Platform::Desktop | crate::platform::Platform::Terminal => {
                if path.exists() {
                    let content = std::fs::read_to_string(path)?;
                    let config: Config =
                        toml::from_str(&content).map_err(SurfDeskError::TomlDeserialization)?;
                    log::info!("Configuration loaded from: {}", path.display());
                    Ok(config)
                } else {
                    log::info!("No configuration file found, using defaults");
                    Ok(Config::default())
                }
            }
            crate::platform::Platform::Web => {
                // Load from localStorage
                #[cfg(target_arch = "wasm32")]
                {
                    if let Ok(window) = web_sys::window() {
                        if let Ok(storage) = window.local_storage() {
                            if let Some(Some(config_str)) = storage.get_item("surfdesk-config") {
                                let config: Config = serde_json::from_str(&config_str)?;
                                log::info!("Configuration loaded from localStorage");
                                return Ok(config);
                            }
                        }
                    }
                }
                Ok(Config::default())
            }
        }
    }

    /// Save configuration to file
    pub async fn save(&mut self) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let platform = crate::current_platform();

        match platform {
            crate::platform::Platform::Desktop | crate::platform::Platform::Terminal => {
                let content = toml::to_string_pretty(&self.config)
                    .map_err(SurfDeskError::TomlSerialization)?;
                std::fs::write(&self.config_path, content)?;
                log::info!("Configuration saved to: {}", self.config_path.display());
            }
            crate::platform::Platform::Web => {
                #[cfg(target_arch = "wasm32")]
                {
                    if let Ok(window) = web_sys::window() {
                        if let Ok(storage) = window.local_storage() {
                            let config_str = serde_json::to_string(&self.config)?;
                            storage.set_item("surfdesk-config", &config_str)?;
                            log::info!("Configuration saved to localStorage");
                        }
                    }
                }
            }
        }

        self.dirty = false;
        Ok(())
    }

    /// Get the current configuration
    pub fn get(&self) -> &Config {
        &self.config
    }

    /// Update configuration
    pub fn update<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Config),
    {
        updater(&mut self.config);
        self.dirty = true;
        Ok(())
    }

    /// Get Solana RPC URL
    pub async fn get_solana_rpc_url(&self) -> Result<String> {
        let network = self.config.solana.default_network;
        Ok(
            if let Some(url) = self
                .config
                .solana
                .custom_endpoints
                .get(&network.to_string())
            {
                url.clone()
            } else {
                network.rpc_url().to_string()
            },
        )
    }

    /// Get project settings
    pub fn get_project_settings(&self) -> &crate::types::ProjectSettings {
        &self.config.project
    }

    /// Get UI settings
    pub fn get_ui_settings(&self) -> &UISettings {
        &self.config.ui
    }

    /// Get logging settings
    pub fn get_logging_settings(&self) -> &LoggingSettings {
        &self.config.logging
    }
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Project settings
    pub project: crate::types::ProjectSettings,
    /// Solana configuration
    pub solana: SolanaConfig,
    /// UI configuration
    pub ui: UISettings,
    /// Logging configuration
    pub logging: LoggingSettings,
    /// Platform-specific settings
    pub platform: PlatformConfig,
    /// Security settings
    pub security: SecurityConfig,
}

/// Solana-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    /// Default network to use
    pub default_network: crate::types::SolanaNetwork,
    /// Custom RPC endpoints
    pub custom_endpoints: HashMap<String, String>,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Commitment level
    pub commitment: CommitmentLevel,
    /// Airdrop configuration
    pub airdrop: AirdropConfig,
}

impl Default for SolanaConfig {
    fn default() -> Self {
        let mut custom_endpoints = HashMap::new();
        custom_endpoints.insert(
            "mainnet-beta".to_string(),
            crate::DEFAULT_SOLANA_RPC_URL.to_string(),
        );
        custom_endpoints.insert("devnet".to_string(), crate::DEVNET_URL.to_string());
        custom_endpoints.insert("testnet".to_string(), crate::TESTNET_URL.to_string());
        custom_endpoints.insert("local".to_string(), crate::LOCAL_VALIDATOR_URL.to_string());

        Self {
            default_network: crate::types::SolanaNetwork::Devnet,
            custom_endpoints,
            connection_timeout: 30,
            commitment: CommitmentLevel::Confirmed,
            airdrop: AirdropConfig::default(),
        }
    }
}

/// Commitment level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

impl CommitmentLevel {
    /// Get the Solana commitment level string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Processed => "processed",
            Self::Confirmed => "confirmed",
            Self::Finalized => "finalized",
        }
    }
}

/// Airdrop configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropConfig {
    /// Whether airdrop is enabled
    pub enabled: bool,
    /// Default airdrop amount in lamports
    pub default_amount: u64,
    /// Maximum daily airdrop amount
    pub max_daily_amount: u64,
    /// Airdrop requests per day limit
    pub requests_per_day: u32,
}

impl Default for AirdropConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_amount: 2_000_000_000,    // 2 SOL
            max_daily_amount: 10_000_000_000, // 10 SOL
            requests_per_day: 5,
        }
    }
}

/// UI configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    /// Theme preference
    pub theme: crate::types::Theme,
    /// Language preference
    pub language: String,
    /// Font size
    pub font_size: u8,
    /// Whether to show notifications
    pub notifications: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Whether to show advanced features
    pub advanced_features: bool,
    /// Window state
    pub window: WindowSettings,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            theme: crate::types::Theme::Auto,
            language: "en".to_string(),
            font_size: 14,
            notifications: true,
            auto_save_interval: 30,
            advanced_features: false,
            window: WindowSettings::default(),
        }
    }
}

/// Window configuration (desktop only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Whether window is maximized
    pub maximized: bool,
    /// Window position x
    pub x: i32,
    /// Window position y
    pub y: i32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            maximized: false,
            x: 100,
            y: 100,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    /// Log level
    pub level: LogLevel,
    /// Whether to log to file
    pub file_logging: bool,
    /// Maximum log file size in MB
    pub max_file_size: u64,
    /// Number of log files to keep
    pub max_files: u32,
    /// Whether to include timestamps
    pub timestamps: bool,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            file_logging: true,
            max_file_size: 10,
            max_files: 5,
            timestamps: true,
        }
    }
}

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Get the log filter string
    pub fn as_filter(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

/// Platform-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformConfig {
    /// Desktop-specific settings
    pub desktop: DesktopConfig,
    /// Web-specific settings
    pub web: WebConfig,
    /// Terminal-specific settings
    pub terminal: TerminalConfig,
}

/// Desktop platform configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopConfig {
    /// Whether to enable system tray
    pub system_tray: bool,
    /// Whether to enable auto-start
    pub auto_start: bool,
    /// Whether to enable global shortcuts
    pub global_shortcuts: bool,
    /// Whether to enable file associations
    pub file_associations: bool,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            system_tray: true,
            auto_start: false,
            global_shortcuts: true,
            file_associations: true,
        }
    }
}

/// Web platform configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    /// Whether to enable PWA features
    pub pwa_enabled: bool,
    /// Whether to enable offline mode
    pub offline_mode: bool,
    /// Service worker version
    pub service_worker_version: String,
    /// Cache settings
    pub cache: CacheConfig,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            pwa_enabled: true,
            offline_mode: true,
            service_worker_version: "1.0.0".to_string(),
            cache: CacheConfig::default(),
        }
    }
}

/// Cache configuration for web platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Whether to enable caching
    pub enabled: bool,
    /// Cache TTL in seconds
    pub ttl: u64,
    /// Maximum cache size in MB
    pub max_size: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl: 3600,    // 1 hour
            max_size: 50, // 50 MB
        }
    }
}

/// Terminal platform configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    /// Whether to enable colors
    pub colors: bool,
    /// Whether to enable Unicode characters
    pub unicode: bool,
    /// Refresh rate in milliseconds
    pub refresh_rate: u64,
    /// Key bindings
    pub key_bindings: HashMap<String, String>,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert("quit".to_string(), "q".to_string());
        key_bindings.insert("help".to_string(), "?".to_string());
        key_bindings.insert("refresh".to_string(), "r".to_string());

        Self {
            colors: true,
            unicode: true,
            refresh_rate: 100,
            key_bindings,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Whether to enable encrypted storage
    pub encrypted_storage: bool,
    /// Session timeout in minutes
    pub session_timeout: u64,
    /// Whether to require authentication
    pub require_auth: bool,
    /// Allowed origins for web platform
    pub allowed_origins: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            encrypted_storage: true,
            session_timeout: 60, // 1 hour
            require_auth: false,
            allowed_origins: vec!["http://localhost:3000".to_string()],
        }
    }
}

/// Initialize configuration service
pub async fn init_config() -> Result<()> {
    let _config_service = ConfigService::new().await?;
    log::info!("Configuration service initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_service_creation() {
        let result = ConfigService::new().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut service = ConfigService::new().await.unwrap();
        let result = service.update(|config| {
            config.ui.font_size = 16;
        });
        assert!(result.is_ok());
        assert_eq!(service.get().ui.font_size, 16);
    }

    #[tokio::test]
    async fn test_config_save() {
        let mut service = ConfigService::new().await.unwrap();
        service
            .update(|config| {
                config.ui.font_size = 18;
            })
            .unwrap();
        let result = service.save().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(
            config.solana.default_network,
            crate::types::SolanaNetwork::Devnet
        );
        assert_eq!(config.ui.theme, crate::types::Theme::Auto);
        assert_eq!(config.logging.level, LogLevel::Info);
    }

    #[test]
    fn test_commitment_level() {
        assert_eq!(CommitmentLevel::Processed.as_str(), "processed");
        assert_eq!(CommitmentLevel::Confirmed.as_str(), "confirmed");
        assert_eq!(CommitmentLevel::Finalized.as_str(), "finalized");
    }

    #[test]
    fn test_log_level() {
        assert_eq!(LogLevel::Error.as_filter(), "error");
        assert_eq!(LogLevel::Info.as_filter(), "info");
        assert_eq!(LogLevel::Debug.as_filter(), "debug");
    }

    #[test]
    fn test_solana_config_default() {
        let config = SolanaConfig::default();
        assert_eq!(config.default_network, crate::types::SolanaNetwork::Devnet);
        assert_eq!(config.connection_timeout, 30);
        assert_eq!(config.commitment, CommitmentLevel::Confirmed);
        assert!(config.custom_endpoints.contains_key("devnet"));
    }

    #[test]
    fn test_ui_settings_default() {
        let settings = UISettings::default();
        assert_eq!(settings.theme, crate::types::Theme::Auto);
        assert_eq!(settings.language, "en");
        assert_eq!(settings.font_size, 14);
        assert!(settings.notifications);
    }

    #[test]
    fn test_logging_settings_default() {
        let settings = LoggingSettings::default();
        assert_eq!(settings.level, LogLevel::Info);
        assert!(settings.file_logging);
        assert_eq!(settings.max_file_size, 10);
        assert_eq!(settings.max_files, 5);
    }
}
