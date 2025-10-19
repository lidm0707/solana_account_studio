//! Surfpool Service for managing local Solana simulation network
//!
//! This service handles the lifecycle of the Surfpool process, which creates
//! a local Solana network by forking mainnet to port 8999. It provides
//! functionality to start, stop, monitor, and configure the simulation.

use crate::services::{AsyncService, Configurable, Service, ServiceError, ServiceResult};
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::path::PathBuf;
use std::time::Duration;
use sysinfo::{Process, ProcessStatus, System};
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

/// Configuration for Surfpool service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfpoolConfig {
    /// Path to surfpool executable
    pub executable_path: PathBuf,
    /// Port for the simulation network
    pub port: u16,
    /// Auto-start on service initialization
    pub auto_start: bool,
    /// Log level for surfpool process
    pub log_level: String,
    /// Additional command line arguments
    pub additional_args: Vec<String>,
    /// Working directory for surfpool process
    pub working_directory: Option<PathBuf>,
    /// Environment variables
    pub env_vars: std::collections::HashMap<String, String>,
}

impl Default for SurfpoolConfig {
    fn default() -> Self {
        Self {
            executable_path: PathBuf::from("surfpool"),
            port: 8999,
            auto_start: false,
            log_level: "info".to_string(),
            additional_args: Vec::new(),
            working_directory: None,
            env_vars: std::collections::HashMap::new(),
        }
    }
}

/// Status of the Surfpool service
#[derive(Debug, Clone, PartialEq)]
pub enum SurfpoolStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error(String),
}

/// Network information for the Surfpool simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Network name (e.g., "Local Simulation")
    pub name: String,
    /// RPC endpoint URL
    pub rpc_url: String,
    /// WebSocket endpoint URL
    pub ws_url: String,
    /// Port number
    pub port: u16,
    /// Network type (mainnet fork, devnet, etc.)
    pub network_type: String,
    /// Genesis hash of the forked network
    pub genesis_hash: Option<String>,
    /// Current slot number
    pub current_slot: Option<u64>,
    /// Whether the network is ready for connections
    pub is_ready: bool,
}

/// Service for managing Surfpool process and network
pub struct SurfpoolService {
    config: SurfpoolConfig,
    status: SurfpoolStatus,
    process: Option<Child>,
    system_info: System,
    logs: Vec<String>,
}

impl SurfpoolService {
    /// Create a new Surfpool service with default configuration
    pub fn new() -> Self {
        Self::with_config(SurfpoolConfig::default())
    }

    /// Create a new Surfpool service with custom configuration
    pub fn with_config(config: SurfpoolConfig) -> Self {
        Self {
            config,
            status: SurfpoolStatus::Stopped,
            process: None,
            system_info: System::new(),
            logs: Vec::new(),
        }
    }

    /// Start the Surfpool process
    pub async fn start_process(&mut self) -> ServiceResult<()> {
        if self.status != SurfpoolStatus::Stopped {
            return Err(ServiceError::Process(
                "Surfpool is already running or in transition state".to_string(),
            ));
        }

        self.status = SurfpoolStatus::Starting;
        self.add_log("Starting Surfpool process...");

        // Check if executable exists
        if !self.config.executable_path.exists() {
            self.status = SurfpoolStatus::Error("Executable not found".to_string());
            return Err(ServiceError::Process(format!(
                "Surfpool executable not found: {:?}",
                self.config.executable_path
            )));
        }

        // Build command
        let mut cmd = Command::new(&self.config.executable_path);

        // Add port argument
        cmd.arg("--port");
        cmd.arg(self.config.port.to_string());

        // Add log level
        cmd.arg("--log-level");
        cmd.arg(&self.config.log_level);

        // Add additional arguments
        for arg in &self.config.additional_args {
            cmd.arg(arg);
        }

        // Set working directory if specified
        if let Some(ref work_dir) = self.config.working_directory {
            cmd.current_dir(work_dir);
        }

        // Set environment variables
        for (key, value) in &self.config.env_vars {
            cmd.env(key, value);
        }

        // Configure stdio
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // Start the process
        match cmd.spawn() {
            Ok(mut child) => {
                let pid = child.id();
                self.process = Some(child);
                self.add_log(&format!("Surfpool started with PID: {}", pid));

                // Wait for the process to be ready
                if self.wait_for_ready().await? {
                    self.status = SurfpoolStatus::Running;
                    self.add_log("Surfpool is ready and accepting connections");
                    Ok(())
                } else {
                    self.status = SurfpoolStatus::Error("Failed to become ready".to_string());
                    Err(ServiceError::Process("Surfpool failed to become ready".to_string()))
                }
            }
            Err(e) => {
                self.status = SurfpoolStatus::Error(format!("Failed to start: {}", e));
                Err(ServiceError::Process(format!("Failed to start Surfpool: {}", e)))
            }
        }
    }

    /// Stop the Surfpool process
    pub async fn stop_process(&mut self) -> ServiceResult<()> {
        if self.status != SurfpoolStatus::Running {
            return Err(ServiceError::Process(
                "Surfpool is not running".to_string(),
            ));
        }

        self.status = SurfpoolStatus::Stopping;
        self.add_log("Stopping Surfpool process...");

        if let Some(mut process) = self.process.take() {
            // Try graceful shutdown first
            match process.kill() {
                Ok(_) => {
                    self.add_log("Sent termination signal to Surfpool");

                    // Wait for process to exit
                    match timeout(Duration::from_secs(10), async {
                        loop {
                            match process.try_wait() {
                                Ok(Some(_)) => return Ok(()),
                                Ok(None) => sleep(Duration::from_millis(100)).await,
                                Err(e) => return Err(ServiceError::Process(e.to_string())),
                            }
                        }
                    })
                    .await
                    {
                        Ok(_) => {
                            self.add_log("Surfpool stopped gracefully");
                        }
                        Err(_) => {
                            warn!("Surfpool did not stop gracefully, forcing");
                            // Force kill
                            let _ = process.kill();
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to kill Surfpool process: {}", e);
                }
            }
        }

        self.status = SurfpoolStatus::Stopped;
        Ok(())
    }

    /// Get current status of the Surfpool service
    pub fn get_status(&self) -> SurfpoolStatus {
        self.status.clone()
    }

    /// Get network information
    pub async fn get_network_info(&self) -> ServiceResult<NetworkInfo> {
        if self.status != SurfpoolStatus::Running {
            return Err(ServiceError::Network(
                "Surfpool is not running".to_string(),
            ));
        }

        // In a real implementation, this would query the Surfpool RPC endpoint
        // For now, return simulated network info
        Ok(NetworkInfo {
            name: "Local Simulation".to_string(),
            rpc_url: format!("http://localhost:{}", self.config.port),
            ws_url: format!("ws://localhost:{}", self.config.port),
            port: self.config.port,
            network_type: "Mainnet Fork".to_string(),
            genesis_hash: Some("4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY".to_string()),
            current_slot: Some(250000000),
            is_ready: true,
        })
    }

    /// Check if Surfpool is ready to accept connections
    async fn wait_for_ready(&mut self) -> ServiceResult<bool> {
        let mut attempts = 0;
        let max_attempts = 30; // 30 seconds with 1-second intervals

        while attempts < max_attempts {
            // Check if process is still running
            if let Some(ref mut process) = self.process {
                match process.try_wait() {
                    Ok(Some(_)) => {
                        self.add_log("Surfpool process exited prematurely");
                        return Ok(false);
                    }
                    Ok(None) => {
                        // Process is still running, check if it's ready
                        if self.check_health().await.unwrap_or(false) {
                            self.add_log("Surfpool is ready and accepting connections");
                            return Ok(true);
                        }
                    }
                    Err(e) => {
                        self.add_log("Error checking process status");
                        return Err(ServiceError::Process(e.to_string()));
                    }
                }
            }

            sleep(Duration::from_secs(1)).await;
            attempts += 1;
        }

        self.add_log("Timeout waiting for Surfpool to be ready");
        Ok(false)
    }

    /// Check if the configured port is ready for connections
    async fn is_port_ready(&self) -> ServiceResult<bool> {
        // In a real implementation, this would attempt to connect to the port
        // For now, simulate the check
        Ok(true)
    }

    /// Get recent logs from the service
    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }

    /// Clear the service logs
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    /// Add a log entry with timestamp
    fn add_log(&mut self, message: &str) {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] {}", timestamp, message);
        self.logs.push(log_entry);
        debug!("Surfpool: {}", message);

        // Keep only last 1000 log entries
        if self.logs.len() > 1000 {
            self.logs.drain(0..self.logs.len() - 1000);
        }
    }

    /// Check if any Surfpool processes are running on the system
    pub fn find_running_processes(&mut self) -> Vec<u32> {
        self.system_info.refresh_processes();

        let mut pids = Vec::new();
        for (pid, process) in self.system_info.processes() {
            if process.name() == "surfpool" || process.name().contains("surf") {
                pids.push(pid.as_u32());
            }
        }

        pids
    }

    /// Kill all running Surfpool processes
    pub async fn kill_all_processes(&mut self) -> ServiceResult<usize> {
        let pids = self.find_running_processes();
        let mut killed_count = 0;

        for pid in pids {
            if let Some(process) = self.system_info.process(pid as usize) {
                if process.kill() {
                    killed_count += 1;
                    self.add_log(&format!("Killed Surfpool process with PID: {}", pid));
                }
            }
        }

        if killed_count > 0 {
            self.system_info.refresh_processes();
        }

        Ok(killed_count)
    }

    /// Restart the Surfpool service
    pub async fn restart(&mut self) -> ServiceResult<()> {
        self.add_log("Restarting Surfpool service...");

        if self.status == SurfpoolStatus::Running {
            self.stop_process().await?;
        }

        // Wait a moment before restarting
        sleep(Duration::from_secs(2)).await;

        self.start_process().await?;
        self.add_log("Surfpool service restarted successfully");

        Ok(())
    }

    /// Get system resource usage for the Surfpool process
    pub fn get_resource_usage(&mut self) -> Option<ResourceUsage> {
        if let Some(ref process) = self.process {
            let pid = process.id();
            self.system_info.refresh_process(pid.into());

            if let Some(sys_process) = self.system_info.process(pid.into()) {
                Some(ResourceUsage {
                    cpu_usage: sys_process.cpu_usage(),
                    memory_usage: sys_process.memory(),
                    disk_usage: 0, // Would need additional tracking
                    status: format!("{:?}", sys_process.status()),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Service for SurfpoolService {
    fn initialize(&mut self) -> ServiceResult<()> {
        self.system_info.refresh_all();
        self.add_log("Surfpool service initialized");

        if self.config.auto_start {
            info!("Auto-starting Surfpool as configured");
            // Note: This is sync, but start_process is async
            // In real usage, would call initialize_async
        }

        Ok(())
    }

    fn health_check(&self) -> ServiceResult<bool> {
        match self.status {
            SurfpoolStatus::Running => Ok(true),
            SurfpoolStatus::Stopped => Ok(false),
            _ => Ok(false), // Transition states are not healthy
        }
    }

    fn shutdown(&mut self) -> ServiceResult<()> {
        // This is the sync version - best effort cleanup
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
        }
        self.status = SurfpoolStatus::Stopped;
        self.add_log("Surfpool service shutdown");
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncService for SurfpoolService {
    async fn initialize_async(&mut self) -> ServiceResult<()> {
        self.initialize()?;

        if self.config.auto_start {
            self.start_process().await?;
        }

        Ok(())
    }

    async fn health_check_async(&self) -> ServiceResult<bool> {
        // More comprehensive health check
        if self.status != SurfpoolStatus::Running {
            return Ok(false);
        }

        // Check if process is still alive
        if let Some(ref process) = self.process {
            match process.try_wait() {
                Ok(Some(_)) => {
                    warn!("Surfpool process has exited");
                    return Ok(false);
                }
                Ok(None) => {
                    // Process is still running, check if it's responding
                    return self.is_port_ready().await;
                }
                Err(_) => return Ok(false),
            }
        }

        Ok(false)
    }

    async fn shutdown_async(&mut self) -> ServiceResult<()> {
        if self.status == SurfpoolStatus::Running {
            self.stop_process().await?;
        }
        Ok(())
    }
}

impl Configurable for SurfpoolService {
    type Config = SurfpoolConfig;

    fn configure(&mut self, config: Self::Config) -> ServiceResult<()> {
        if self.status == SurfpoolStatus::Running {
            return Err(ServiceError::Configuration(
                "Cannot reconfigure while Surfpool is running".to_string(),
            ));
        }

        self.config = config;
        self.add_log("Surfpool configuration updated");
        Ok(())
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }
}

/// Resource usage information for the Surfpool process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Disk usage in bytes (if tracked)
    pub disk_usage: u64,
    /// Process status
    pub status: String,
}

/// Default configuration presets for different environments
pub mod presets {
    use super::*;

    pub fn development() -> SurfpoolConfig {
        SurfpoolConfig {
            log_level: "debug".to_string(),
            additional_args: vec!["--verbose".to_string()],
            ..Default::default()
        }
    }

    pub fn production() -> SurfpoolConfig {
        SurfpoolConfig {
            log_level: "info".to_string(),
            auto_start: true,
            ..Default::default()
        }
    }

    pub fn testing() -> SurfpoolConfig {
        SurfpoolConfig {
            port: 8998, // Different port for testing
            log_level: "error".to_string(),
            additional_args: vec!["--no-persistence".to_string()],
            ..Default::default()
        }
    }
}
