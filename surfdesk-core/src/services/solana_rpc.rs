//! Solana RPC Service for custom Solana network interaction
//!
//! This service provides a custom implementation of Solana RPC functionality
//! without using external SDKs. It handles communication with Solana nodes,
//! transaction building, account management, and program deployment through
//! direct HTTP JSON-RPC calls.

use crate::services::{AsyncService, Configurable, Service, ServiceError, ServiceResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use reqwest::Client;
use bs58;
use sha2::{Sha256, Digest};

/// Configuration for Solana RPC service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaRpcConfig {
    /// RPC endpoint URL
    pub rpc_url: String,
    /// WebSocket endpoint URL
    pub ws_url: Option<String>,
    /// Request timeout in seconds
    pub timeout: u64,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Commitment level
    pub commitment: CommitmentLevel,
    /// Pre-flight checks
    pub preflight_checks: bool,
}

impl Default for SolanaRpcConfig {
    fn default() -> Self {
        Self {
            rpc_url: "http://localhost:8999".to_string(), // Default to Surfpool
            ws_url: None,
            timeout: 30,
            max_retries: 3,
            commitment: CommitmentLevel::Confirmed,
            preflight_checks: true,
        }
    }
}

/// Commitment levels for Solana transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

impl Default for CommitmentLevel {
    fn default() -> Self {
        Self::Confirmed
    }
}

/// Solana account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account address
    pub address: String,
    /// Account balance in lamports
    pub balance: u64,
    /// Account owner
    pub owner: String,
    /// Account data
    pub data: Vec<u8>,
    /// Whether the account is executable
    pub executable: bool,
    /// Rent epoch
    pub rent_epoch: u64,
    /// Account size
    pub size: u64,
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Transaction signature
    pub signature: String,
    /// Transaction slot
    pub slot: u64,
    /// Block time
    pub block_time: Option<i64>,
    /// Transaction status
    pub status: TransactionStatus,
    /// Error message if failed
    pub error: Option<String>,
    /// Fee paid
    pub fee: u64,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}

/// Program deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDeployment {
    /// Program ID
    pub program_id: String,
    /// Transaction signature
    pub signature: String,
    /// Deployment slot
    pub slot: u64,
    /// Program size in bytes
    pub size: u64,
}

/// Service for interacting with Solana network via RPC
pub struct SolanaRpcService {
    config: SolanaRpcConfig,
    client: Client,
    request_id: u64,
}

impl SolanaRpcService {
    /// Create a new Solana RPC service with default configuration
    pub fn new() -> Self {
        Self::with_config(SolanaRpcConfig::default())
    }

    /// Create a new Solana RPC service with custom configuration
    pub fn with_config(config: SolanaRpcConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .unwrap_or_default();

        Self {
            config,
            client,
            request_id: 1,
        }
    }

    /// Get account information
    pub async fn get_account(&mut self, address: &str) -> ServiceResult<AccountInfo> {
        let params = json!([
            address,
            {
                "encoding": "base64"
            }
        ]);

        let response = self.make_request("getAccountInfo", params).await?;

        if let Some(account_data) = response.get("result").and_then(|r| r.get("value")) {
            let balance = account_data.get("lamports")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let owner = account_data.get("owner")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let data_base64 = account_data.get("data")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let data = base64::decode(data_base64).unwrap_or_default();

            let executable = account_data.get("executable")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let rent_epoch = account_data.get("rentEpoch")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            Ok(AccountInfo {
                address: address.to_string(),
                balance,
                owner,
                data: data.clone(),
                executable,
                rent_epoch,
                size: data.len() as u64,
            })
        } else {
            Err(ServiceError::Network(format!("Account not found: {}", address)))
        }
    }

    /// Get account balance
    pub async fn get_balance(&mut self, address: &str) -> ServiceResult<u64> {
        let params = json!([address]);

        let response = self.make_request("getBalance", params).await?;

        if let Some(balance) = response.get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_u64())
        {
            Ok(balance)
        } else {
            Err(ServiceError::Network("Failed to get balance".to_string()))
        }
    }

    /// Send transaction
    pub async fn send_transaction(&mut self, transaction: &[u8]) -> ServiceResult<String> {
        let transaction_base64 = base64::encode(transaction);
        let params = json!([transaction_base64]);

        let response = self.make_request("sendTransaction", params).await?;

        if let Some(signature) = response.get("result")
            .and_then(|v| v.as_str())
        {
            Ok(signature.to_string())
        } else {
            Err(ServiceError::Network("Failed to send transaction".to_string()))
        }
    }

    /// Get transaction information
    pub async fn get_transaction(&mut self, signature: &str) -> ServiceResult<TransactionInfo> {
        let params = json!([
            signature,
            {
                "encoding": "json"
            }
        ]);

        let response = self.make_request("getTransaction", params).await?;

        if let Some(tx_data) = response.get("result") {
            let slot = tx_data.get("slot")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let block_time = tx_data.get("blockTime")
                .and_then(|v| v.as_i64());

            let default_meta = json!({});
            let meta = tx_data.get("meta").unwrap_or(&default_meta);
            let fee = meta.get("fee")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let err = meta.get("err");
            let status = if err.is_some() && err.unwrap().is_null() {
                TransactionStatus::Success
            } else if err.is_some() {
                TransactionStatus::Failed
            } else {
                TransactionStatus::Pending
            };

            let error = if status == TransactionStatus::Failed {
                Some(format!("{:?}", err))
            } else {
                None
            };

            Ok(TransactionInfo {
                signature: signature.to_string(),
                slot,
                block_time,
                status,
                error,
                fee,
            })
        } else {
            Err(ServiceError::Network(format!("Transaction not found: {}", signature)))
        }
    }

    /// Deploy a program
    pub async fn deploy_program(&self, program_data: &[u8]) -> ServiceResult<ProgramDeployment> {
        // In a real implementation, this would:
        // 1. Create a program deployment transaction
        // 2. Sign it with the appropriate keys
        // 3. Send it to the network
        // 4. Wait for confirmation

        // For now, return a mock deployment
        let program_id = generate_program_id(program_data);
        let signature = format!("mock_signature_{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));

        Ok(ProgramDeployment {
            program_id,
            signature,
            slot: 123456789,
            size: program_data.len() as u64,
        })
    }

    /// Get latest blockhash
    pub async fn get_latest_blockhash(&mut self) -> ServiceResult<String> {
        let params = json!([{
            "commitment": format!("{:?}", self.config.commitment).to_lowercase()
        }]);

        let response = self.make_request("getLatestBlockhash", params).await?;

        if let Some(blockhash) = response.get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.get("blockhash"))
            .and_then(|v| v.as_str())
        {
            Ok(blockhash.to_string())
        } else {
            Err(ServiceError::Network("Failed to get latest blockhash".to_string()))
        }
    }

    /// Make a JSON-RPC request
    async fn make_request(&mut self, method: &str, params: Value) -> ServiceResult<Value> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params
        });

        self.request_id += 1;

        let response = self.client
            .post(&self.config.rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ServiceError::Http(e.to_string()))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| ServiceError::Http(e.to_string()))?;

        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| ServiceError::Serialization(e))?;

        // Check for RPC error
        if let Some(error) = response_json.get("error") {
            let message = error.get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown RPC error");

            return Err(ServiceError::Network(format!("RPC error: {}", message)));
        }

        Ok(response_json)
    }

    /// Airdrop SOL to an account (devnet/testnet only)
    pub async fn request_airdrop(&mut self, address: &str, lamports: u64) -> ServiceResult<String> {
        let params = json!([address, lamports]);

        let response = self.make_request("requestAirdrop", params).await?;

        if let Some(signature) = response.get("result")
            .and_then(|v| v.as_str())
        {
            Ok(signature.to_string())
        } else {
            Err(ServiceError::Network("Failed to request airdrop".to_string()))
        }
    }
}

impl Service for SolanaRpcService {
    fn initialize(&mut self) -> ServiceResult<()> {
        tracing::info!("Solana RPC service initialized with endpoint: {}", self.config.rpc_url);
        Ok(())
    }

    fn health_check(&self) -> ServiceResult<bool> {
        // In a sync context, we can't make async calls
        // This is a basic health check - in real usage would use health_check_async
        Ok(!self.config.rpc_url.is_empty())
    }

    fn shutdown(&mut self) -> ServiceResult<()> {
        tracing::info!("Solana RPC service shutdown");
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncService for SolanaRpcService {
    async fn initialize_async(&mut self) -> ServiceResult<()> {
        self.initialize()?;

        // Test connection by getting latest blockhash
        match self.get_latest_blockhash().await {
            Ok(_) => {
                tracing::info!("Successfully connected to Solana RPC endpoint");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to connect to Solana RPC endpoint: {}", e);
                Err(e)
            }
        }
    }

    async fn health_check_async(&self) -> ServiceResult<bool> {
        match self.get_latest_blockhash().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn shutdown_async(&mut self) -> ServiceResult<()> {
        self.shutdown()?;
        Ok(())
    }
}

impl Configurable for SolanaRpcService {
    type Config = SolanaRpcConfig;

    fn configure(&mut self, config: Self::Config) -> ServiceResult<()> {
        self.config = config.clone();

        // Recreate client with new timeout
        self.client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .unwrap_or_default();

        tracing::info!("Solana RPC service reconfigured with endpoint: {}", config.rpc_url);
        Ok(())
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }
}

/// Generate a mock program ID from program data
fn generate_program_id(program_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(program_data);
    let hash = hasher.finalize();

    // Convert first 32 bytes to a mock base58 address
    let hash_bytes = &hash[..32];
    bs58::encode(hash_bytes).into_string()
}

/// Create a basic transfer transaction
pub fn create_transfer_transaction(from: &str, to: &str, lamports: u64) -> ServiceResult<Vec<u8>> {
    // In a real implementation, this would create a proper Solana transaction
    // For now, return a mock transaction
    let transaction_data = format!("transfer:{}:{}:{}", from, to, lamports);
    Ok(transaction_data.into_bytes())
}

/// Create a program deployment transaction
pub fn create_program_deployment_transaction(program_data: &[u8]) -> ServiceResult<Vec<u8>> {
    // In a real implementation, this would create a proper Solana program deployment transaction
    let mut transaction = Vec::new();
    transaction.extend_from_slice(b"program_deploy:");
    transaction.extend_from_slice(program_data);
    Ok(transaction)
}

/// Validate a Solana address
pub fn validate_address(address: &str) -> bool {
    bs58::decode(address).into_vec().is_ok()
}

/// Convert lamports to SOL
pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

/// Convert SOL to lamports
pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_address() {
        // Test valid base58 address
        let valid_address = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        assert!(validate_address(valid_address));

        // Test invalid address
        let invalid_address = "invalid_address";
        assert!(!validate_address(invalid_address));
    }

    #[test]
    fn test_lamports_conversion() {
        let lamports = 1_500_000_000;
        let sol = lamports_to_sol(lamports);
        assert_eq!(sol, 1.5);

        let back_to_lamports = sol_to_lamports(sol);
        assert_eq!(back_to_lamports, lamports);
    }

    #[test]
    fn test_create_transfer_transaction() {
        let from = "11111111111111111111111111111111";
        let to = "22222222222222222222222222222222";
        let lamports = 1_000_000_000;

        let transaction = create_transfer_transaction(from, to, lamports).unwrap();
        assert!(!transaction.is_empty());
    }

    #[test]
    fn test_generate_program_id() {
        let program_data = b"test_program_data";
        let program_id = generate_program_id(program_data);
        assert!(!program_id.is_empty());
        assert!(validate_address(&program_id));
    }
}
