//! Environment Management for SurfPool
//!
//! This module provides cross-platform environment switching and configuration
//! for different Solana development environments (local devnet, mainnet fork, custom).

use crate::error::SurfDeskError;
use crate::platform::Platform;
use crate::surfpool::{PresetAccount, SurfPoolConfig};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Helper function to get display text for environment type
fn config_type_display(config: &EnvironmentConfig) -> &'static str {
    match config.r#type {
        EnvironmentType::LocalDevnet => "Local Devnet",
        EnvironmentType::MainnetFork => "Mainnet Fork",
        EnvironmentType::Custom => "Custom",
    }
}

/// Helper function to get display text for environment type (web version)
fn config_type_display_web(config: &EnvironmentConfig) -> &'static str {
    match config.r#type {
        EnvironmentType::LocalDevnet => "Local",
        EnvironmentType::MainnetFork => "Fork",
        EnvironmentType::Custom => "Custom",
    }
}

/// Environment types supported by SurfPool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentType {
    /// Local development network
    LocalDevnet,
    /// Mainnet fork
    MainnetFork,
    /// Custom environment
    Custom,
}

impl Default for EnvironmentType {
    fn default() -> Self {
        Self::LocalDevnet
    }
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Environment type
    pub r#type: EnvironmentType,
    /// RPC port
    pub rpc_port: u16,
    /// WebSocket port
    pub ws_port: u16,
    /// Ledger path
    pub ledger_path: String,
    /// Accounts path
    pub accounts_path: String,
    /// Preset accounts for the environment
    pub preset_accounts: Vec<PresetAccount>,
    /// Mainnet fork specific settings
    pub fork_settings: Option<ForkSettings>,
    /// Custom genesis configuration
    pub custom_genesis: Option<serde_json::Value>,
    /// Whether to enable MCP
    pub enable_mcp: bool,
    /// Whether to enable Anchor project detection
    pub anchor_project: bool,
}

/// Mainnet fork settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkSettings {
    /// Fork URL
    pub fork_url: String,
    /// Fork slot (None for latest)
    pub fork_slot: Option<u64>,
    /// Warp slot
    pub warp_slot: Option<u64>,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            r#type: EnvironmentType::LocalDevnet,
            rpc_port: 8899,
            ws_port: 8900,
            ledger_path: dirs::cache_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("surfdesk")
                .join("ledger")
                .to_string_lossy()
                .to_string(),
            accounts_path: dirs::cache_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("surfdesk")
                .join("accounts")
                .to_string_lossy()
                .to_string(),
            preset_accounts: vec![
                PresetAccount {
                    pubkey: "11111111111111111111111111111112".to_string(),
                    lamports: 1_000_000_000_000,
                    tokens: None,
                },
                PresetAccount {
                    pubkey: "11111111111111111111111111111113".to_string(),
                    lamports: 1_000_000_000_000,
                    tokens: None,
                },
            ],
            fork_settings: None,
            custom_genesis: None,
            enable_mcp: true,
            anchor_project: true,
        }
    }
}

/// Environment manager for cross-platform environment switching
#[derive(Clone)]
pub struct EnvironmentManager {
    /// Current platform
    platform: Platform,
    /// Available environments
    environments: Arc<RwLock<HashMap<String, EnvironmentConfig>>>,
    /// Currently active environment
    active_environment: Arc<RwLock<Option<String>>>,
}

impl EnvironmentManager {
    /// Create a new environment manager
    pub fn new(platform: Platform) -> Self {
        let mut environments = HashMap::new();

        // Add default environments
        environments.insert("local-devnet".to_string(), EnvironmentConfig::default());
        environments.insert(
            "mainnet-fork".to_string(),
            EnvironmentConfig {
                r#type: EnvironmentType::MainnetFork,
                rpc_port: 8900,
                ws_port: 8901,
                ledger_path: dirs::cache_dir()
                    .unwrap_or_else(std::env::temp_dir)
                    .join("surfdesk")
                    .join("mainnet-fork-ledger")
                    .to_string_lossy()
                    .to_string(),
                accounts_path: dirs::cache_dir()
                    .unwrap_or_else(std::env::temp_dir)
                    .join("surfdesk")
                    .join("mainnet-fork-accounts")
                    .to_string_lossy()
                    .to_string(),
                preset_accounts: vec![],
                fork_settings: Some(ForkSettings {
                    fork_url: "https://api.mainnet-beta.solana.com".to_string(),
                    fork_slot: None,
                    warp_slot: None,
                }),
                custom_genesis: None,
                enable_mcp: true,
                anchor_project: false, // Typically disabled for mainnet forks
            },
        );

        Self {
            platform,
            environments: Arc::new(RwLock::new(environments)),
            active_environment: Arc::new(RwLock::new(None)),
        }
    }

    /// Get all available environments
    pub async fn get_environments(&self) -> HashMap<String, EnvironmentConfig> {
        self.environments.read().await.clone()
    }

    /// Get a specific environment configuration
    pub async fn get_environment(&self, name: &str) -> Option<EnvironmentConfig> {
        self.environments.read().await.get(name).cloned()
    }

    /// Add or update an environment
    pub async fn save_environment(
        &self,
        name: String,
        config: EnvironmentConfig,
    ) -> Result<(), SurfDeskError> {
        let mut environments = self.environments.write().await;
        environments.insert(name, config);
        Ok(())
    }

    /// Delete an environment
    pub async fn delete_environment(&self, name: &str) -> Result<(), SurfDeskError> {
        let mut environments = self.environments.write().await;
        environments.remove(name);

        // If the deleted environment was active, clear the active environment
        let mut active = self.active_environment.write().await;
        if let Some(active_env) = active.as_ref() {
            if active_env == name {
                *active = None;
            }
        }

        Ok(())
    }

    /// Switch to a different environment
    pub async fn switch_environment(&self, name: &str) -> Result<EnvironmentConfig, SurfDeskError> {
        let environments = self.environments.read().await;
        let config = environments
            .get(name)
            .ok_or_else(|| SurfDeskError::platform(format!("Environment '{}' not found", name)))?
            .clone();
        drop(environments);

        let mut active = self.active_environment.write().await;
        *active = Some(name.to_string());
        drop(active);

        Ok(config)
    }

    /// Get the currently active environment
    pub async fn get_active_environment(&self) -> Option<(String, EnvironmentConfig)> {
        let active_name = self.active_environment.read().await.clone()?;
        let environments = self.environments.read().await;
        let config = environments.get(&active_name)?.clone();
        Some((active_name, config))
    }

    /// Validate environment configuration
    pub fn validate_config(&self, config: &EnvironmentConfig) -> Result<(), SurfDeskError> {
        // Check port availability
        if config.rpc_port == config.ws_port {
            return Err(SurfDeskError::platform(
                "RPC and WebSocket ports cannot be the same",
            ));
        }

        // Validate ports are in valid range
        if config.rpc_port < 1024 {
            return Err(SurfDeskError::platform(
                "RPC port must be between 1024 and 65535",
            ));
        }
        if config.ws_port < 1024 || config.ws_port > 65535 {
            return Err(SurfDeskError::platform(
                "WebSocket port must be between 1024 and 65535",
            ));
        }

        // Validate fork settings if present
        if let Some(fork_settings) = &config.fork_settings {
            if fork_settings.fork_url.is_empty() {
                return Err(SurfDeskError::platform("Fork URL cannot be empty"));
            }
        }

        // Validate preset accounts
        for account in &config.preset_accounts {
            if account.pubkey.is_empty() {
                return Err(SurfDeskError::platform(
                    "Preset account public key cannot be empty",
                ));
            }
        }

        Ok(())
    }

    /// Convert environment configuration to SurfPool configuration
    pub fn to_surfpool_config(&self, config: &EnvironmentConfig) -> SurfPoolConfig {
        let fork_url = config.fork_settings.as_ref().map(|fs| fs.fork_url.clone());
        let fork_slot = config.fork_settings.as_ref().and_then(|fs| fs.fork_slot);

        SurfPoolConfig {
            rpc_port: config.rpc_port,
            ws_port: config.ws_port,
            ledger_path: config.ledger_path.clone(),
            accounts_path: config.accounts_path.clone(),
            auto_start: false,
            resource_limits: Default::default(),
            fork_url,
            fork_slot,
            enable_mcp: config.enable_mcp,
            anchor_project: config.anchor_project,
            preset_accounts: config.preset_accounts.clone(),
        }
    }

    /// Generate command arguments for surfpool based on environment
    pub fn generate_surfpool_args(&self, config: &EnvironmentConfig) -> Vec<String> {
        let mut args = vec!["start".to_string()];

        // Add RPC port
        args.push("--rpc-port".to_string());
        args.push(config.rpc_port.to_string());

        // Add WebSocket port
        args.push("--ws-port".to_string());
        args.push(config.ws_port.to_string());

        // Add ledger path
        args.push("--ledger".to_string());
        args.push(config.ledger_path.clone());

        // Enable MCP if requested
        if config.enable_mcp {
            args.push("--mcp".to_string());
        }

        // Enable Anchor project detection
        if config.anchor_project {
            args.push("--anchor".to_string());
        }

        // Add preset accounts
        for account in &config.preset_accounts {
            args.push("--account".to_string());
            args.push(format!("{}:{}", account.pubkey, account.lamports));
        }

        // Add fork arguments if applicable
        if let Some(fork_settings) = &config.fork_settings {
            args.push("--fork".to_string());
            args.push(fork_settings.fork_url.clone());

            if let Some(slot) = fork_settings.fork_slot {
                args.push("--fork-slot".to_string());
                args.push(slot.to_string());
            }

            if let Some(warp_slot) = fork_settings.warp_slot {
                args.push("--warp-slot".to_string());
                args.push(warp_slot.to_string());
            }
        }

        args
    }
}

/// Hook to use environment manager in Dioxus components
pub fn use_environment_manager(platform: Platform) -> Signal<EnvironmentManager> {
    use_signal(|| EnvironmentManager::new(platform))
}

/// Environment selector component for desktop platforms
#[component]
pub fn EnvironmentSelector(
    platform: Platform,
    on_environment_change: Option<EventHandler<String>>,
) -> Element {
    let mut selected_environment = use_signal(|| "local-devnet".to_string());

    let on_change = move |event: Event<FormData>| {
        let value = event.value();
        selected_environment.set(value.clone());

        if let Some(handler) = on_environment_change {
            handler.call(value);
        }
    };

    rsx! {
        div { class: "environment-selector",
            label { class: "block text-sm font-medium mb-2",
                "Development Environment"
            }
            select {
                class: "w-full p-2 border rounded-md bg-white dark:bg-gray-800",
                value: "{selected_environment}",
                onchange: on_change,

                option {
                    value: "local-devnet",
                    "Local Devnet"
                }
                option {
                    value: "mainnet-fork",
                    "Mainnet Fork"
                }
                option {
                    value: "custom",
                    "Custom"
                }
            }
        }
    }
}

/// Environment selector component for web platform
#[component]
pub fn WebEnvironmentSelector(
    platform: Platform,
    on_environment_change: Option<EventHandler<String>>,
) -> Element {
    let mut selected_environment = use_signal(|| "local-devnet".to_string());

    let on_change = move |event: Event<FormData>| {
        let value = event.value();
        selected_environment.set(value.clone());

        if let Some(handler) = on_environment_change {
            handler.call(value);
        }
    };

    rsx! {
        div { class: "web-environment-selector p-4 bg-white dark:bg-gray-800 rounded-lg shadow",
            label { class: "block text-sm font-medium mb-2",
                "Development Environment"
            }
            select {
                class: "w-full p-2 border rounded-md",
                value: "{selected_environment}",
                onchange: on_change,

                option {
                    value: "local-devnet",
                    "Local"
                }
                option {
                    value: "mainnet-fork",
                    "Fork"
                }
                option {
                    value: "custom",
                    "Custom"
                }
            }

            div { class: "mt-2 text-xs text-gray-500",
                "Select environment for local Solana development"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_config_default() {
        let config = EnvironmentConfig::default();
        assert_eq!(config.r#type, EnvironmentType::LocalDevnet);
        assert_eq!(config.rpc_port, 8899);
        assert_eq!(config.ws_port, 8900);
        assert_eq!(config.preset_accounts.len(), 2);
        assert!(config.enable_mcp);
        assert!(config.anchor_project);
    }

    #[test]
    fn test_environment_manager_creation() {
        let manager = EnvironmentManager::new(Platform::Desktop);
        // Should have default environments
    }

    #[test]
    fn test_config_validation() {
        let manager = EnvironmentManager::new(Platform::Desktop);

        // Valid config should pass
        let valid_config = EnvironmentConfig::default();
        assert!(manager.validate_config(&valid_config).is_ok());

        // Invalid config (same ports) should fail
        let invalid_config = EnvironmentConfig {
            rpc_port: 8899,
            ws_port: 8899,
            ..Default::default()
        };
        assert!(manager.validate_config(&invalid_config).is_err());
    }

    #[tokio::test]
    async fn test_environment_switching() {
        let manager = EnvironmentManager::new(Platform::Web);

        // Should be able to switch to local-devnet
        let config = manager.switch_environment("local-devnet").await;
        assert!(config.is_ok());

        // Should get active environment
        let active = manager.get_active_environment().await;
        assert!(active.is_some());
        assert_eq!(active.unwrap().0, "local-devnet");
    }

    #[test]
    fn test_to_surfpool_config() {
        let manager = EnvironmentManager::new(Platform::Desktop);
        let env_config = EnvironmentConfig::default();
        let surfpool_config = manager.to_surfpool_config(&env_config);

        assert_eq!(surfpool_config.rpc_port, env_config.rpc_port);
        assert_eq!(surfpool_config.ws_port, env_config.ws_port);
        assert_eq!(surfpool_config.enable_mcp, env_config.enable_mcp);
        assert_eq!(surfpool_config.anchor_project, env_config.anchor_project);
    }

    #[test]
    fn test_generate_surfpool_args() {
        let manager = EnvironmentManager::new(Platform::Desktop);
        let config = EnvironmentConfig::default();
        let args = manager.generate_surfpool_args(&config);

        assert!(args.contains(&"start".to_string()));
        assert!(args.contains(&"--rpc-port".to_string()));
        assert!(args.contains(&"--mcp".to_string()));
        assert!(args.contains(&"--anchor".to_string()));
    }
}
