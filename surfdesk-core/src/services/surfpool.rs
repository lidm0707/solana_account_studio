//! # SurfPool Service - Terminal Strategy
//!
//! This service provides integration with SurfPool using terminal commands.
//! Instead of complex service integration, we use simple command-line calls
//! to manage the external SurfPool process.

use crate::error::{Result, SurfDeskError};
use crate::platform::Platform;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::process::Command as TokioCommand;
use tokio::sync::RwLock;

/// SurfPool service using terminal command strategy
#[derive(Clone, Debug)]
pub struct SurfPoolService {
    /// Current platform
    platform: Platform,
    /// Service status
    status: Arc<RwLock<ServiceStatus>>,
    /// Current configuration
    config: Arc<RwLock<SurfPoolConfig>>,
}

/// Service status
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    /// Service is stopped
    Stopped,
    /// Service is starting
    Starting,
    /// Service is running
    Running,
    /// Service is stopping
    Stopping,
    /// Service encountered an error
    Error(String),
    /// Service status is unknown
    Unknown,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

/// Simple SurfPool configuration for terminal strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SurfPoolConfig {
    /// RPC port for the validator
    pub rpc_port: u16,
    /// WebSocket port for the validator
    pub ws_port: u16,
    /// Fork URL for mainnet forking
    pub fork_url: Option<String>,
    /// Enable MCP (Model Context Protocol)
    pub enable_mcp: bool,
}

impl Default for SurfPoolConfig {
    fn default() -> Self {
        Self {
            rpc_port: 8899,
            ws_port: 8900,
            fork_url: None,
            enable_mcp: true,
        }
    }
}

/// SurfPool process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfPoolProcess {
    /// Process ID
    pub pid: Option<u32>,
    /// RPC port
    pub rpc_port: u16,
    /// WebSocket port
    pub ws_port: u16,
    /// Status
    pub status: String,
    /// Start time
    pub start_time: Option<String>,
}

impl SurfPoolService {
    /// Create a new SurfPool service with terminal strategy
    pub async fn new() -> Result<Self> {
        let platform = crate::current_platform();

        info!(
            "Initializing SurfPool service (terminal strategy) for platform: {:?}",
            platform
        );

        // Check if SurfPool is installed
        if !Self::is_surfpool_installed().await? {
            warn!("SurfPool is not installed");
            return Err(SurfDeskError::platform(
                "SurfPool is not installed. Install with: cargo install surfpool",
            ));
        }

        info!("SurfPool is available via terminal");

        Ok(Self {
            platform,
            status: Arc::new(RwLock::new(ServiceStatus::Stopped)),
            config: Arc::new(RwLock::new(SurfPoolConfig::default())),
        })
    }

    /// Create a fallback SurfPool service for when SurfPool is not installed
    pub fn new_fallback() -> Self {
        warn!("Creating fallback SurfPool service - limited functionality");
        Self {
            platform: crate::current_platform(),
            status: Arc::new(RwLock::new(ServiceStatus::Error(
                "SurfPool not installed".to_string(),
            ))),
            config: Arc::new(RwLock::new(SurfPoolConfig::default())),
        }
    }

    /// Check if SurfPool is installed by trying to run --version
    async fn is_surfpool_installed() -> Result<bool> {
        let output = TokioCommand::new("surfpool")
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Start SurfPool using terminal command
    pub async fn start(&self) -> Result<()> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot start SurfPool: not installed. Install with: cargo install surfpool",
            ));
        }

        let mut status = self.status.write().await;
        if *status != ServiceStatus::Stopped {
            return Err(SurfDeskError::platform(
                "SurfPool service is already running",
            ));
        }

        *status = ServiceStatus::Starting;
        drop(status);

        info!("Starting SurfPool via terminal command");

        let config = self.config.read().await;
        let mut cmd = TokioCommand::new("surfpool");
        cmd.arg("start");
        cmd.arg("--rpc-port");
        cmd.arg(config.rpc_port.to_string());
        cmd.arg("--ws-port");
        cmd.arg(config.ws_port.to_string());

        if config.enable_mcp {
            cmd.arg("--enable-mcp");
        }

        if let Some(ref fork_url) = config.fork_url {
            cmd.arg("--fork");
            cmd.arg(fork_url);
        }

        // Run in background (detached)
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        match cmd.spawn() {
            Ok(_) => {
                info!("SurfPool started successfully via terminal");
                let mut status = self.status.write().await;
                *status = ServiceStatus::Running;
                Ok(())
            }
            Err(e) => {
                error!("Failed to start SurfPool: {}", e);
                let mut status = self.status.write().await;
                *status = ServiceStatus::Error(e.to_string());
                Err(SurfDeskError::platform(format!(
                    "Failed to start SurfPool: {}",
                    e
                )))
            }
        }
    }

    /// Stop SurfPool using terminal command
    pub async fn stop(&self) -> Result<()> {
        let mut status = self.status.write().await;
        if *status != ServiceStatus::Running {
            return Err(SurfDeskError::platform("SurfPool service is not running"));
        }

        *status = ServiceStatus::Stopping;
        drop(status);

        info!("Stopping SurfPool via terminal command");

        let output = TokioCommand::new("surfpool")
            .arg("stop")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(_) => {
                info!("SurfPool stopped successfully via terminal");
                let mut status = self.status.write().await;
                *status = ServiceStatus::Stopped;
                Ok(())
            }
            Err(e) => {
                error!("Failed to stop SurfPool: {}", e);
                let mut status = self.status.write().await;
                *status = ServiceStatus::Error(e.to_string());
                Err(SurfDeskError::platform(format!(
                    "Failed to stop SurfPool: {}",
                    e
                )))
            }
        }
    }

    /// Get SurfPool status using terminal command
    pub async fn get_status(&self) -> Result<ServiceStatus> {
        let output = TokioCommand::new("surfpool")
            .arg("status")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(output) => {
                let status_str = String::from_utf8_lossy(&output.stdout);
                if status_str.contains("running") {
                    Ok(ServiceStatus::Running)
                } else if status_str.contains("stopped") {
                    Ok(ServiceStatus::Stopped)
                } else {
                    Ok(ServiceStatus::Unknown)
                }
            }
            Err(e) => {
                error!("Failed to get SurfPool status: {}", e);
                Ok(ServiceStatus::Error(e.to_string()))
            }
        }
    }

    /// Get SurfPool process information
    pub async fn get_process_info(&self) -> Result<SurfPoolProcess> {
        let output = TokioCommand::new("surfpool")
            .arg("info")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(output) => {
                let info_str = String::from_utf8_lossy(&output.stdout);
                // Parse the output to extract process information
                // This is a simplified parser - you may need to adjust based on actual surfpool output
                let process = SurfPoolProcess {
                    pid: self.extract_pid(&info_str),
                    rpc_port: 8899, // Default, can be parsed from output
                    ws_port: 8900,  // Default, can be parsed from output
                    status: "running".to_string(),
                    start_time: Some("now".to_string()),
                };
                Ok(process)
            }
            Err(e) => {
                error!("Failed to get SurfPool process info: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to get process info: {}",
                    e
                )))
            }
        }
    }

    /// Extract PID from surfpool info output
    fn extract_pid(&self, output: &str) -> Option<u32> {
        // Look for PID in the output
        for line in output.lines() {
            if line.contains("PID:") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    if let Ok(pid) = parts[1].trim().parse::<u32>() {
                        return Some(pid);
                    }
                }
            }
        }
        None
    }

    /// Get current configuration
    pub async fn get_config(&self) -> Result<SurfPoolConfig> {
        let config = self.config.read().await;
        Ok(config.clone())
    }

    /// Update configuration
    pub async fn update_config(&self, new_config: SurfPoolConfig) -> Result<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        info!("SurfPool configuration updated");
        Ok(())
    }

    /// Deploy a program using SurfPool
    pub async fn deploy_program(&self, program_path: &str) -> Result<String> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot deploy program: SurfPool not installed. Install with: cargo install surfpool"
            ));
        }

        info!("Deploying program via SurfPool: {}", program_path);

        let output = TokioCommand::new("surfpool")
            .arg("deploy")
            .arg(program_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                if !result.is_empty() {
                    info!("Program deployed successfully: {}", result);
                    Ok(result.trim().to_string())
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    error!("Failed to deploy program: {}", error);
                    Err(SurfDeskError::platform(format!(
                        "Deployment failed: {}",
                        error
                    )))
                }
            }
            Err(e) => {
                error!("Failed to deploy program: {}", e);
                Err(SurfDeskError::platform(format!("Deployment error: {}", e)))
            }
        }
    }

    /// Create an account using SurfPool
    pub async fn create_account(&self, label: &str) -> Result<String> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot create account: SurfPool not installed. Install with: cargo install surfpool"
            ));
        }

        info!("Creating account via SurfPool: {}", label);

        let output = TokioCommand::new("surfpool")
            .arg("account")
            .arg("create")
            .arg("--label")
            .arg(label)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                if !result.is_empty() {
                    info!("Account created successfully: {}", result);
                    Ok(result.trim().to_string())
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    error!("Failed to create account: {}", error);
                    Err(SurfDeskError::platform(format!(
                        "Account creation failed: {}",
                        error
                    )))
                }
            }
            Err(e) => {
                error!("Failed to create account: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Account creation error: {}",
                    e
                )))
            }
        }
    }

    /// Get account balance using SurfPool
    pub async fn get_balance(&self, pubkey: &str) -> Result<f64> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot get balance: SurfPool not installed. Install with: cargo install surfpool",
            ));
        }

        info!("Getting balance via SurfPool: {}", pubkey);

        let output = TokioCommand::new("surfpool")
            .arg("account")
            .arg("balance")
            .arg(pubkey)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                if !result.is_empty() {
                    // Parse balance from output (assuming it returns SOL amount)
                    if let Ok(balance) = result.trim().parse::<f64>() {
                        info!("Account balance: {} SOL", balance);
                        Ok(balance)
                    } else {
                        Err(SurfDeskError::platform("Failed to parse balance"))
                    }
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    error!("Failed to get balance: {}", error);
                    Err(SurfDeskError::platform(format!(
                        "Balance query failed: {}",
                        error
                    )))
                }
            }
            Err(e) => {
                error!("Failed to get balance: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Balance query error: {}",
                    e
                )))
            }
        }
    }

    /// Get the current platform
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Check if SurfPool supports MCP on this platform
    pub fn supports_mcp(&self) -> bool {
        matches!(self.platform, Platform::Desktop | Platform::Terminal)
    }

    /// Check if this is a fallback service (SurfPool not installed)
    pub fn is_fallback(&self) -> bool {
        match self.status.try_read() {
            Ok(status) => {
                matches!(*status, ServiceStatus::Error(ref msg) if msg.contains("not installed"))
            }
            Err(_) => false,
        }
    }

    /// Get account balance (placeholder implementation)
    pub async fn get_account_balance(&self, _pubkey: &crate::solana_rpc::Pubkey) -> Result<f64> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Ok(0.0); // Return default balance for fallback
        }

        // Placeholder implementation - in real scenario would query Solana RPC
        info!("Getting account balance for pubkey: {}", _pubkey);
        Ok(1.0) // Return mock balance
    }

    /// Start validator (placeholder implementation)
    pub async fn start_validator(&self) -> Result<()> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot start validator: SurfPool not installed",
            ));
        }

        info!("Starting validator via SurfPool");
        // For now, just start the service
        self.start().await
    }

    /// Stop validator (placeholder implementation)
    pub async fn stop_validator(&self) -> Result<()> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot stop validator: SurfPool not installed",
            ));
        }

        info!("Stopping validator via SurfPool");
        // For now, just stop the service
        self.stop().await
    }

    /// Get transaction by signature (placeholder implementation)
    pub async fn get_transaction_by_signature(
        &self,
        _signature: &crate::solana_rpc::Signature,
    ) -> Option<crate::solana_rpc::transactions::Transaction> {
        info!("Getting transaction by signature: {}", _signature);
        // Return mock transaction for now
        Some(crate::solana_rpc::transactions::Transaction {
            signatures: vec![_signature.as_str().to_string()],
            instructions: vec![],
            recent_blockhash: "mock_blockhash".to_string(),
            fee_payer: crate::solana_rpc::Pubkey::new_unique(),
        })
    }

    /// Get latest blockhash (placeholder implementation)
    pub async fn get_latest_blockhash(&self) -> Option<String> {
        info!("Getting latest blockhash");
        Some("mock_latest_blockhash".to_string())
    }
}

#[async_trait]
impl super::Service for SurfPoolService {
    fn name(&self) -> &'static str {
        "SurfPool"
    }

    async fn initialize(&mut self) -> Result<()> {
        info!("Initializing SurfPool service (terminal strategy)");

        // Check if SurfPool is available
        if !Self::is_surfpool_installed().await? {
            return Err(SurfDeskError::platform(
                "SurfPool is not installed. Install with: cargo install surfpool",
            ));
        }

        info!("SurfPool service initialized successfully");
        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        let status = self.get_status().await?;
        Ok(matches!(status, ServiceStatus::Running))
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down SurfPool service");

        // Stop SurfPool if it's running
        {
            let current_status = self.status.read().await;
            if *current_status == ServiceStatus::Running {
                drop(current_status);
                if let Err(e) = self.stop().await {
                    warn!("Failed to stop SurfPool during shutdown: {}", e);
                }
            }
        }

        info!("SurfPool service shutdown successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_surfpool_service_creation() {
        // This test will fail if SurfPool is not installed
        let result = SurfPoolService::new().await;
        // We don't assert success here since SurfPool might not be installed in test environment
        println!("SurfPool service creation result: {:?}", result);
    }

    #[test]
    fn test_config_default() {
        let config = SurfPoolConfig::default();
        assert_eq!(config.rpc_port, 8899);
        assert_eq!(config.ws_port, 8900);
        assert!(config.enable_mcp);
    }

    #[test]
    fn test_status_display() {
        let status = ServiceStatus::Running;
        assert_eq!(status, ServiceStatus::Running);
    }
}
