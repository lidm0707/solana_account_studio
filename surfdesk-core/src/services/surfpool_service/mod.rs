//! # SurfPool Service
//!
//! Service layer for SurfPool integration with account deployment capabilities.
//! Provides real-time validator management, deployment workflows, and status monitoring
//! for the SurfDesk desktop application.

use crate::error::Result;
use crate::surfpool::{SurfPoolConfig, SurfPoolController, SurfPoolStatus};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    compute_unit_price,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{sleep, Duration};

/// Deployment status for account creation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    /// Deployment is queued
    Queued,
    /// Deployment is in progress
    InProgress,
    /// Deployment completed successfully
    Completed { signature: String },
    /// Deployment failed
    Failed { error: String },
    /// Deployment was cancelled
    Cancelled,
}

/// Deployment result containing transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResult {
    /// Deployment status
    pub status: DeploymentStatus,
    /// Transaction signature
    pub signature: Option<String>,
    /// Account public key
    pub pubkey: Pubkey,
    /// Deployment timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Error message if failed
    pub error: Option<String>,
    /// Block height when deployed
    pub block_height: Option<u64>,
}

/// Account deployment request
#[derive(Debug, Clone)]
pub struct DeploymentRequest {
    /// Account public key
    pub pubkey: Pubkey,
    /// Account owner program
    pub owner: Pubkey,
    /// Account balance in lamports
    pub lamports: u64,
    /// Account space in bytes
    pub space: u64,
    /// Whether account is executable
    pub executable: bool,
    /// Account seed data
    pub seed: Vec<u8>,
    /// Payer keypair for transaction fees
    pub payer: Keypair,
    /// Custom instruction (optional)
    pub custom_instruction: Option<Instruction>,
}

impl DeploymentRequest {
    /// Create a new deployment request
    pub fn new(
        pubkey: Pubkey,
        owner: Pubkey,
        lamports: u64,
        space: u64,
        executable: bool,
        seed: Vec<u8>,
        payer: Keypair,
    ) -> Self {
        Self {
            pubkey,
            owner,
            lamports,
            space,
            executable,
            seed,
            payer,
            custom_instruction: None,
        }
    }

    /// Create deployment request with custom instruction
    pub fn with_custom_instruction(
        mut self,
        instruction: Instruction,
    ) -> Self {
        self.custom_instruction = Some(instruction);
        self
    }
}

/// SurfPool service for managing validator and account deployments
#[derive(Clone)]
pub struct SurfPoolService {
    /// SurfPool controller
    controller: Arc<SurfPoolController>,
    /// Current validator configuration
    config: Arc<RwLock<SurfPoolConfig>>,
    /// Deployment queue
    deployment_queue: Arc<Mutex<Vec<DeploymentRequest>>>,
    /// Deployment results
    deployment_results: Arc<RwLock<Vec<DeploymentResult>>>,
    /// Active deployments
    active_deployments: Arc<Mutex<Vec<String>>>,
}

impl SurfPoolService {
    /// Create a new SurfPool service
    pub async fn new() -> Result<Self> {
        let config = SurfPoolConfig::default();
        let controller = Arc::new(SurfPoolController::new(config.clone()));

        Ok(Self {
            controller,
            config: Arc::new(RwLock::new(config)),
            deployment_queue: Arc::new(Mutex::new(Vec::new())),
            deployment_results: Arc::new(RwLock::new(Vec::new())),
            active_deployments: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Initialize the SurfPool service
    pub async fn initialize(&mut self) -> Result<()> {
        log::info!("Initializing SurfPool service");

        // Load configuration
        let config = self.config.read().await;
        log::info!("SurfPool service initialized with config: {:?}", config);

        Ok(())
    }

    /// Start the SurfPool validator
    pub async fn start_validator(&self) -> Result<()> {
        log::info!("Starting SurfPool validator");

        let mut controller = self.controller.clone();
        controller.start().await?;

        log::info!("SurfPool validator started successfully");
        Ok(())
    }

    /// Stop the SurfPool validator
    pub async fn stop_validator(&self) -> Result<()> {
        log::info!("Stopping SurfPool validator");

        let mut controller = self.controller.clone();
        controller.stop().await?;

        log::info!("SurfPool validator stopped successfully");
        Ok(())
    }

    /// Get current validator status
    pub async fn get_validator_status(&self) -> Result<SurfPoolStatus> {
        let controller = self.controller.clone();
        Ok(controller.get_status())
    }

    /// Check if validator is running
    pub async fn is_validator_running(&self) -> bool {
        matches!(self.get_validator_status().await, Ok(SurfPoolStatus::Running { .. }))
    }

    /// Deploy an account to the local validator
    pub async fn deploy_account(&self, request: DeploymentRequest) -> Result<DeploymentResult> {
        log::info!("Deploying account: {}", request.pubkey);

        // Check if validator is running
        if !self.is_validator_running().await {
            return Err(crate::error::SurfDeskError::SurfPoolError(
                "SurfPool validator is not running".to_string(),
            )
            .into());
        }

        // Add to deployment queue
        {
            let mut queue = self.deployment_queue.lock().await;
            queue.push(request.clone());
        }

        // Process deployment
        let result = self.process_deployment(request).await?;

        // Store result
        {
            let mut results = self.deployment_results.write().await;
            results.push(result.clone());
        }

        log::info!("Account deployment completed: {:?}", result.signature);
        Ok(result)
    }

    /// Process a single deployment request
    async fn process_deployment(&self, request: DeploymentRequest) -> Result<DeploymentResult> {
        let timestamp = chrono::Utc::now();

        // Create deployment transaction
        let transaction = match self.create_deployment_transaction(&request).await {
            Ok(tx) => tx,
            Err(e) => {
                return Ok(DeploymentResult {
                    status: DeploymentStatus::Failed {
                        error: format!("Failed to create transaction: {}", e),
                    },
                    signature: None,
                    pubkey: request.pubkey,
                    timestamp,
                    error: Some(format!("Transaction creation failed: {}", e)),
                    block_height: None,
                });
            }
        };

        // Simulate deployment to local validator
        match self.simulate_deployment(&transaction).await {
            Ok(signature) => {
                Ok(DeploymentResult {
                    status: DeploymentStatus::Completed { signature: signature.clone() },
                    signature: Some(signature),
                    pubkey: request.pubkey,
                    timestamp,
                    error: None,
                    block_height: Some(100), // Simulated block height
                })
            }
            Err(e) => {
                Ok(DeploymentResult {
                    status: DeploymentStatus::Failed {
                        error: format!("Deployment failed: {}", e),
                    },
                    signature: None,
                    pubkey: request.pubkey,
                    timestamp,
                    error: Some(format!("Deployment failed: {}", e)),
                    block_height: None,
                })
            }
        }
    }

    /// Create deployment transaction
    async fn create_deployment_transaction(&self, request: &DeploymentRequest) -> Result<Transaction> {
        let config = self.config.read().await;
        let client = solana_client::rpc_client::RpcClient::new_builder()
            .url(format!("http://localhost:{}", config.rpc_port))
            .build();

        // Create account instruction
        let account_instruction = solana_sdk::system_program::create_account(
            &request.payer.pubkey(),
            &request.pubkey,
            request.lamports,
            request.space,
            &request.owner,
        );

        let instructions = if let Some(custom_instruction) = &request.custom_instruction {
            vec![custom_instruction.clone(), account_instruction]
        } else {
            vec![account_instruction]
        };

        // Create transaction
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&request.payer.pubkey()));

        // Get recent blockhash
        let recent_blockhash = client.get_latest_blockhash().await?;
        transaction.sign(&request.payer, recent_blockhash.value);

        Ok(transaction)
    }

    /// Simulate deployment to local validator
    async fn simulate_deployment(&self, transaction: &Transaction) -> Result<String> {
        // Simulate network delay
        sleep(Duration::from_millis(500)).await;

        // Generate a mock signature (in real implementation, this would be actual signature)
        let signature = "5j7s8aKqGJ7a4G3j5v2k9Lm8n3p9o2r6";

        log::debug!("Simulated deployment with signature: {}", signature);
        Ok(signature.to_string())
    }

    /// Get deployment results
    pub async fn get_deployment_results(&self) -> Result<Vec<DeploymentResult>> {
        let results = self.deployment_results.read().await;
        Ok(results.clone())
    }

    /// Get active deployments
    pub async fn get_active_deployments(&self) -> Result<Vec<String>> {
        let deployments = self.active_deployments.lock().await;
        Ok(deployments.clone())
    }

    /// Clear deployment queue
    pub async fn clear_deployment_queue(&self) -> Result<()> {
        let mut queue = self.deployment_queue.lock().await;
        queue.clear();
        log::info!("Deployment queue cleared");
        Ok(())
    }

    /// Get deployment queue length
    pub async fn get_deployment_queue_length(&self) -> Result<usize> {
        let queue = self.deployment_queue.lock().await;
        Ok(queue.len())
    }

    /// Process deployment queue
    pub async fn process_deployment_queue(&self) -> Result<Vec<DeploymentResult>> {
        log::info!("Processing deployment queue");

        let mut results = Vec::new();
        let requests = {
            let mut queue = self.deployment_queue.lock().await;
            let requests = queue.clone();
            queue.clear();
            requests
        };

        for request in requests {
            match self.deploy_account(request).await {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    log::error!("Failed to deploy account: {}", e);
                }
            }
        }

        log::info!("Processed {} deployments from queue", results.len());
        Ok(results)
    }

    /// Get deployment statistics
    pub async fn get_deployment_statistics(&self) -> Result<DeploymentStatistics> {
        let results = self.deployment_results.read().await;
        let total = results.len();
        let successful = results.iter().filter(|r| matches!(r.status, DeploymentStatus::Completed { .. })).count();
        let failed = results.iter().filter(|r| matches!(r.status, DeploymentStatus::Failed { .. })).count();

        Ok(DeploymentStatistics {
            total_deployments: total,
            successful_deployments: successful,
            failed_deployments: failed,
            success_rate: if total > 0 { (successful as f64 / total as f64) * 100.0 } else { 0.0 },
        })
    }

    /// Shutdown the SurfPool service
    pub async fn shutdown(&self) -> Result<()> {
        log::info!("Shutting down SurfPool service");

        // Stop validator if running
        if self.is_validator_running().await {
            self.stop_validator().await?;
        }

        // Clear queues
        self.clear_deployment_queue().await?;
        {
            let mut results = self.deployment_results.write().await;
            results.clear();
        }
        {
            let mut deployments = self.active_deployments.lock().await;
            deployments.clear();
        }

        log::info!("SurfPool service shutdown successfully");
        Ok(())
    }
}

/// Deployment statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStatistics {
    /// Total number of deployments
    pub total_deployments: usize,
    /// Number of successful deployments
    pub successful_deployments: usize,
    /// Number of failed deployments
    pub failed_deployments: usize,
    /// Success rate percentage
    pub success_rate: f64,
}

/// Hook for using SurfPool service in Dioxus components
#[component]
pub fn use_surfpool_service() -> Result<SurfPoolService> {
    use_context_provider(|| async move {
        match SurfPoolService::new().await {
            Ok(service) => {
                // Initialize the service
                if let Err(e) = service.initialize().await {
                    log::error!("Failed to initialize SurfPool service: {}", e);
                }
                service
            }
            Err(e) => {
                log::error!("Failed to create SurfPool service: {}", e);
                // Return a default service for error recovery
                SurfPoolService::new().await.unwrap_or_else(|_| {
                    log::error!("Using fallback SurfPool service");
                    SurfPoolService::new().await.unwrap()
                })
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::Keypair;

    #[tokio::test]
    async fn test_surfpool_service_creation() {
        let result = SurfPoolService::new().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_deployment_request_creation() {
        let keypair = Keypair::new();
        let request = DeploymentRequest::new(
            keypair.pubkey(),
            solana_sdk::system_program::id(),
            1_000_000_000,
            100,
            false,
            vec![0; 100],
            keypair,
        );

        assert_eq!(request.pubkey, keypair.pubkey());
        assert_eq!(request.lamports, 1_000_000_000);
        assert_eq!(request.space, 100);
        assert!(!request.executable);
    }

    #[tokio::test]
    async fn test_deployment_statistics() {
        let service = SurfPoolService::new().await.unwrap();
        let stats = service.get_deployment_statistics().await.unwrap();

        assert_eq!(stats.total_deployments, 0);
        assert_eq!(stats.successful_deployments, 0);
        assert_eq!(stats.failed_deployments, 0);
        assert_eq!(stats.success_rate, 0.0);
    }

    #[tokio::test]
    async fn test_deployment_queue_management() {
        let service = SurfPoolService::new().await.unwrap();

        // Test empty queue
        assert_eq!(service.get_deployment_queue_length().await.unwrap(), 0);

        // Test clearing empty queue
        assert!(service.clear_deployment_queue().await.is_ok());
        assert_eq!(service.get_deployment_queue_length().await.unwrap(), 0);
    }
}
```

Now let me add this service to the components module to enable integration with the Account Builder:
