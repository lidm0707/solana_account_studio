//! SurfPool: Multi-Platform Solana Development Controller
//!
//! This module provides cross-platform process management for local Solana validators
//! and development environments, supporting desktop, web, and terminal platforms.

pub mod environment;

use crate::error::SurfDeskError;
use crate::platform::Platform;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{sleep, Duration};

/// Cross-platform SurfPool controller for managing Solana validators
#[derive(Clone)]
pub struct SurfPoolController {
    /// Platform-specific implementation
    platform: Platform,
    /// Running validator process
    process: Arc<Mutex<Option<Child>>>,
    /// Current configuration
    config: Arc<RwLock<SurfPoolConfig>>,
    /// Current status
    status: Arc<RwLock<ControllerStatus>>,
}

/// Configuration for SurfPool controller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfPoolConfig {
    /// RPC port for the validator
    pub rpc_port: u16,
    /// WebSocket port for the validator
    pub ws_port: u16,
    /// Ledger directory path
    pub ledger_path: String,
    /// Accounts directory path
    pub accounts_path: String,
    /// Whether to auto-start the validator
    pub auto_start: bool,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Fork URL for mainnet forking
    pub fork_url: Option<String>,
    /// Fork slot for mainnet forking
    pub fork_slot: Option<u64>,
    /// Whether to enable MCP (Model Context Protocol)
    pub enable_mcp: bool,
    /// Anchor project detection
    pub anchor_project: bool,
    /// Preset accounts for the environment
    pub preset_accounts: Vec<PresetAccount>,
}

/// Resource limits for the validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory in MB
    pub max_memory_mb: u32,
    /// Maximum CPU percentage
    pub max_cpu_percent: u32,
    /// Maximum disk space in GB
    pub max_disk_gb: u32,
}

/// Preset account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetAccount {
    /// Public key
    pub pubkey: String,
    /// Initial lamports
    pub lamports: u64,
    /// Optional token account data
    pub tokens: Option<Vec<TokenAccount>>,
}

/// Token account data for preset accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAccount {
    /// Token mint address
    pub mint: String,
    /// Token amount
    pub amount: u64,
}

/// Controller status
#[derive(Debug, Clone, PartialEq)]
pub enum ControllerStatus {
    /// Controller is stopped
    Stopped,
    /// Controller is starting
    Starting,
    /// Controller is running
    Running,
    /// Controller is stopping
    Stopping,
    /// Controller encountered an error
    Error(String),
}

impl Default for ControllerStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 2048,
            max_cpu_percent: 80,
            max_disk_gb: 10,
        }
    }
}

impl Default for SurfPoolConfig {
    fn default() -> Self {
        Self {
            rpc_port: 8899,
            ws_port: 8900,
            ledger_path: dirs::cache_dir()
                .unwrap_or_else(|| std::env::temp_dir())
                .join("surfdesk")
                .join("ledger")
                .to_string_lossy()
                .to_string(),
            accounts_path: dirs::cache_dir()
                .unwrap_or_else(|| std::env::temp_dir())
                .join("surfdesk")
                .join("accounts")
                .to_string_lossy()
                .to_string(),
            auto_start: false,
            resource_limits: ResourceLimits::default(),
            fork_url: None,
            fork_slot: None,
            enable_mcp: true,
            anchor_project: true,
            preset_accounts: vec![PresetAccount {
                pubkey: "11111111111111111111111111111112".to_string(),
                lamports: 1_000_000_000_000,
                tokens: None,
            }],
        }
    }
}

impl SurfPoolController {
    /// Create a new SurfPool controller
    pub async fn new(platform: Platform) -> Result<Self, SurfDeskError> {
        let config = SurfPoolConfig::default();

        Ok(Self {
            platform,
            process: Arc::new(Mutex::new(None)),
            config: Arc::new(RwLock::new(config)),
            status: Arc::new(RwLock::new(ControllerStatus::Stopped)),
        })
    }

    /// Create a new controller with custom configuration
    pub async fn with_config(
        platform: Platform,
        config: SurfPoolConfig,
    ) -> Result<Self, SurfDeskError> {
        Ok(Self {
            platform,
            process: Arc::new(Mutex::new(None)),
            config: Arc::new(RwLock::new(config)),
            status: Arc::new(RwLock::new(ControllerStatus::Stopped)),
        })
    }

    /// Start the Solana validator
    pub async fn start(&self) -> Result<(), SurfDeskError> {
        let mut status = self.status.write().await;
        if *status != ControllerStatus::Stopped {
            return Err(SurfDeskError::platform("Controller is already running"));
        }

        *status = ControllerStatus::Starting;
        drop(status);

        let config = self.config.read().await.clone();

        // Ensure directories exist
        std::fs::create_dir_all(&config.ledger_path).map_err(|e| {
            SurfDeskError::platform(format!("Failed to create ledger directory: {}", e))
        })?;
        std::fs::create_dir_all(&config.accounts_path).map_err(|e| {
            SurfDeskError::platform(format!("Failed to create accounts directory: {}", e))
        })?;

        // Build the surfpool command
        let mut cmd = match self.platform {
            Platform::Desktop | Platform::Terminal => {
                let mut cmd = Command::new("surfpool");
                cmd.arg("start");

                // Add RPC port
                cmd.arg("--rpc-port").arg(config.rpc_port.to_string());

                // Add WebSocket port
                cmd.arg("--ws-port").arg(config.ws_port.to_string());

                // Add ledger path
                cmd.arg("--ledger").arg(&config.ledger_path);

                // Add fork configuration if specified
                if let Some(ref fork_url) = config.fork_url {
                    cmd.arg("--fork").arg(fork_url);

                    if let Some(fork_slot) = config.fork_slot {
                        cmd.arg("--fork-slot").arg(fork_slot.to_string());
                    }
                }

                // Enable MCP if requested
                if config.enable_mcp {
                    cmd.arg("--mcp");
                }

                // Enable Anchor project detection
                if config.anchor_project {
                    cmd.arg("--anchor");
                }

                // Add preset accounts
                for account in &config.preset_accounts {
                    cmd.arg("--account")
                        .arg(format!("{}:{}", account.pubkey, account.lamports));
                }

                cmd
            }
            Platform::Web => {
                // On web platform, we'll simulate a validator
                return self.start_web_validator().await;
            }
        };

        // Start the process
        let child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| SurfDeskError::platform(format!("Failed to start surfpool: {}", e)))?;

        let mut process_guard = self.process.lock().await;
        *process_guard = Some(child);
        drop(process_guard);

        // Wait a moment for SurfPool to initialize
        sleep(Duration::from_millis(1000)).await;

        // Update status
        let mut status = self.status.write().await;
        *status = ControllerStatus::Running;

        Ok(())
    }

    /// Start a simulated validator for web platform
    async fn start_web_validator(&self) -> Result<(), SurfDeskError> {
        // For web platform, we simulate a validator with a timeout
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let mut status = self.status.write().await;
        *status = ControllerStatus::Running;

        Ok(())
    }

    /// Stop the Solana validator
    pub async fn stop(&self) -> Result<(), SurfDeskError> {
        let mut status = self.status.write().await;
        if *status != ControllerStatus::Running {
            return Err(SurfDeskError::platform(
                "SurfPool controller is not running",
            ));
        }

        *status = ControllerStatus::Stopping;
        drop(status);

        let mut process_guard = self.process.lock().await;
        if let Some(mut child) = process_guard.take() {
            match self.platform {
                Platform::Desktop | Platform::Terminal => {
                    // Try graceful shutdown first
                    if let Err(e) = child.kill() {
                        log::warn!(
                            "Failed to gracefully stop surfpool: {}, forcing termination",
                            e
                        );
                    }

                    child.wait().map_err(|e| {
                        SurfDeskError::platform(format!("Failed to wait for surfpool: {}", e))
                    })?;
                }
                Platform::Web => {
                    // Simulate stopping
                    sleep(Duration::from_millis(200)).await;
                }
            }
        }
        drop(process_guard);

        let mut status = self.status.write().await;
        *status = ControllerStatus::Stopped;

        Ok(())
    }

    /// Get the current status
    pub async fn get_status(&self) -> ControllerStatus {
        self.status.read().await.clone()
    }

    /// Get the current configuration
    pub async fn get_config(&self) -> SurfPoolConfig {
        self.config.read().await.clone()
    }

    /// Update the configuration
    pub async fn update_config(&self, new_config: SurfPoolConfig) -> Result<(), SurfDeskError> {
        let status = self.status.read().await;
        if *status == ControllerStatus::Running {
            return Err(SurfDeskError::platform(
                "Cannot update config while SurfPool is running",
            ));
        }
        drop(status);

        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }

    /// Check if the validator is healthy
    pub async fn health_check(&self) -> Result<bool, SurfDeskError> {
        let status = self.status.read().await;
        if *status != ControllerStatus::Running {
            return Ok(false);
        }
        drop(status);

        match self.platform {
            Platform::Desktop | Platform::Terminal => {
                // Check if process is still running
                let mut process_guard = self.process.lock().await;
                if let Some(child) = process_guard.as_mut() {
                    match child.try_wait() {
                        Ok(Some(_)) => Ok(false), // Process has exited
                        Ok(None) => Ok(true),     // Process is still running
                        Err(_) => Ok(false),      // Error checking status
                    }
                } else {
                    Ok(false)
                }
            }
            Platform::Web => {
                // Simulate health check
                Ok(true)
            }
        }
    }

    /// Get validator metrics
    pub async fn get_metrics(&self) -> Result<ValidatorMetrics, SurfDeskError> {
        let status = self.status.read().await;
        if *status != ControllerStatus::Running {
            return Err(SurfDeskError::platform("SurfPool is not running"));
        }
        drop(status);

        match self.platform {
            Platform::Desktop | Platform::Terminal => {
                // Collect actual metrics from the system
                // TODO: Implement actual metrics collection from SurfPool RPC
                Ok(ValidatorMetrics {
                    uptime_seconds: 0,    // Would need to track start time
                    memory_usage_mb: 512, // Would need to monitor actual usage
                    cpu_percent: 15.0,
                    disk_usage_gb: 0.5,
                    connected_peers: 0, // Local SurfPool has no external peers
                    slots_processed: 0, // Would need to query RPC
                    transaction_count: 0,
                    fork_height: None,  // Only for fork environments
                    accounts_loaded: 0, // Would need to query from SurfPool
                })
            }
            Platform::Web => {
                // Return simulated metrics
                Ok(ValidatorMetrics {
                    uptime_seconds: 0,
                    memory_usage_mb: 256,
                    cpu_percent: 10.0,
                    disk_usage_gb: 0.1,
                    connected_peers: 0,
                    slots_processed: 0,
                    transaction_count: 0,
                    fork_height: None,
                    accounts_loaded: 0,
                })
            }
        }
    }
}

/// Validator metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorMetrics {
    /// How long the validator has been running
    pub uptime_seconds: u64,
    /// Memory usage in MB
    pub memory_usage_mb: u32,
    /// CPU usage percentage
    pub cpu_percent: f32,
    /// Disk usage in GB
    pub disk_usage_gb: f32,
    /// Number of connected peers
    pub connected_peers: u32,
    /// Number of slots processed
    pub slots_processed: u64,
    /// Number of transactions processed
    pub transaction_count: u64,
    /// Current fork height (for mainnet forks)
    pub fork_height: Option<u64>,
    /// Number of accounts loaded
    pub accounts_loaded: u64,
}

/// Check if SurfPool is installed and available
pub async fn check_surfpool_installation() -> Result<bool, SurfDeskError> {
    let output = Command::new("surfpool")
        .arg("--version")
        .output()
        .map_err(|_| SurfDeskError::platform("SurfPool is not installed"))?;

    Ok(output.status.success())
}

/// Install SurfPool using cargo (if not available)
pub async fn install_surfpool() -> Result<(), SurfDeskError> {
    // Check if already installed
    if check_surfpool_installation().await.is_ok() {
        return Ok(());
    }

    // Try to install using cargo
    let output = Command::new("cargo")
        .args(&["install", "surfpool"])
        .output()
        .map_err(|e| SurfDeskError::platform(format!("Failed to install SurfPool: {}", e)))?;

    if !output.status.success() {
        return Err(SurfDeskError::platform(
            "Failed to install SurfPool. Please install manually: cargo install surfpool",
        ));
    }

    Ok(())
}

/// Hook to use SurfPool controller in Dioxus components
pub fn use_surfpool_controller(platform: Platform) -> Signal<SurfPoolController> {
    let controller = use_signal(|| {
        // Create a blocking runtime for initialization
        let rt = tokio::runtime::Runtime::new().unwrap();
        // Return a placeholder - actual controller will be set up later
        rt.block_on(async { SurfPoolController::new(platform).await.unwrap() })
    });

    controller
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_controller_creation() {
        let controller = SurfPoolController::new(Platform::Desktop).await;
        assert!(controller.is_ok());
    }

    #[tokio::test]
    async fn test_config_default() {
        let config = SurfPoolConfig::default();
        assert_eq!(config.rpc_port, 8899);
        assert_eq!(config.ws_port, 8900);
        assert!(config.enable_mcp);
        assert!(config.anchor_project);
        assert_eq!(config.preset_accounts.len(), 1);
    }

    #[tokio::test]
    async fn test_status_transitions() {
        let controller = SurfPoolController::new(Platform::Web).await.unwrap();

        // Initial status should be stopped
        assert_eq!(controller.get_status().await, ControllerStatus::Stopped);

        // Web validator should start
        assert!(controller.start().await.is_ok());
        assert_eq!(controller.get_status().await, ControllerStatus::Running);

        // Should be able to stop
        assert!(controller.stop().await.is_ok());
        assert_eq!(controller.get_status().await, ControllerStatus::Stopped);
    }

    #[tokio::test]
    async fn test_surfpool_installation_check() {
        // This test will fail if surfpool is not installed
        // In real tests, you might want to mock this
        let result = check_surfpool_installation().await;
        // Don't assert here since surfpool might not be installed in test environment
        match result {
            Ok(is_installed) => println!("SurfPool installation status: {}", is_installed),
            Err(_) => println!("SurfPool not available (expected in test environment)"),
        }
    }

    #[test]
    fn test_preset_account_serialization() {
        let account = PresetAccount {
            pubkey: "11111111111111111111111111111112".to_string(),
            lamports: 1_000_000_000,
            tokens: None,
        };

        // Test serialization
        let json = serde_json::to_string(&account).unwrap();
        let deserialized: PresetAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(account.pubkey, deserialized.pubkey);
        assert_eq!(account.lamports, deserialized.lamports);
    }

    #[test]
    fn test_validator_metrics() {
        let metrics = ValidatorMetrics {
            uptime_seconds: 3600,
            memory_usage_mb: 512,
            cpu_percent: 25.5,
            disk_usage_gb: 1.2,
            connected_peers: 5,
            slots_processed: 1000,
            transaction_count: 50,
            fork_height: Some(150_000_000),
            accounts_loaded: 25,
        };

        assert_eq!(metrics.uptime_seconds, 3600);
        assert_eq!(metrics.memory_usage_mb, 512);
        assert_eq!(metrics.fork_height, Some(150_000_000));
    }
}
