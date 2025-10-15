//! # Solana Service Module
//!
//! This module provides Solana blockchain integration for the SurfDesk application.
//! It handles RPC connections, account management, transaction operations, and
//! program interactions across all platforms.

use crate::error::{Result, SurfDeskError};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature,
    transaction::Transaction,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Solana service for blockchain interactions
pub struct SolanaService {
    /// RPC client
    rpc_client: Arc<RpcClient>,
    /// Connection configuration
    config: SolanaConfig,
    /// Connection status
    status: Arc<RwLock<ConnectionStatus>>,
    /// Cached account data
    account_cache: Arc<RwLock<std::collections::HashMap<String, CachedAccount>>>,
}

impl std::fmt::Debug for SolanaService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolanaService")
            .field("config", &self.config)
            .field("connection_status", &"Connected") // Don't try to debug the RwLock
            .field("cached_accounts", &"Some entries") // Don't try to debug the HashMap
            .finish()
    }
}

/// Solana service configuration
#[derive(Debug, Clone)]
pub struct SolanaConfig {
    /// RPC URL
    pub rpc_url: String,
    /// Commitment level
    pub commitment: CommitmentLevel,
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Whether to use caching
    pub use_cache: bool,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
}

/// Commitment level for transactions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

impl CommitmentLevel {
    /// Convert to Solana commitment config
    pub fn to_commitment_config(self) -> CommitmentConfig {
        match self {
            Self::Processed => CommitmentConfig::processed(),
            Self::Confirmed => CommitmentConfig::confirmed(),
            Self::Finalized => CommitmentConfig::finalized(),
        }
    }
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// Cached account data
#[derive(Debug, Clone)]
struct CachedAccount {
    /// Account data
    account: solana_sdk::account::Account,
    /// Cache timestamp
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl SolanaService {
    /// Create a new Solana service
    pub async fn new(rpc_url: String) -> Result<Self> {
        let config = SolanaConfig {
            rpc_url: rpc_url.clone(),
            commitment: CommitmentLevel::Confirmed,
            timeout: 30,
            use_cache: true,
            cache_ttl: 60,
        };

        Self::with_config(config).await
    }

    /// Create a new Solana service with custom configuration
    pub async fn with_config(config: SolanaConfig) -> Result<Self> {
        log::info!("Initializing Solana service with URL: {}", config.rpc_url);

        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            config.rpc_url.clone(),
            config.commitment.to_commitment_config(),
        ));

        let service = Self {
            rpc_client,
            config,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            account_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        };

        // Test connection
        service.test_connection().await?;

        log::info!("Solana service initialized successfully");
        Ok(service)
    }

    /// Test connection to the Solana network
    async fn test_connection(&self) -> Result<()> {
        *self.status.write().await = ConnectionStatus::Connecting;

        match tokio::task::spawn_blocking({
            let client = self.rpc_client.clone();
            move || client.get_latest_blockhash()
        })
        .await
        {
            Ok(Ok(_)) => {
                *self.status.write().await = ConnectionStatus::Connected;
                log::info!("Successfully connected to Solana network");
                Ok(())
            }
            Ok(Err(e)) => {
                *self.status.write().await = ConnectionStatus::Error;
                Err(SurfDeskError::solana_sdk(format!(
                    "Connection test failed: {}",
                    e
                )))
            }
            Err(e) => {
                *self.status.write().await = ConnectionStatus::Error;
                Err(SurfDeskError::internal(format!(
                    "Connection test panicked: {}",
                    e
                )))
            }
        }
    }

    /// Get current connection status
    pub async fn get_status(&self) -> ConnectionStatus {
        *self.status.read().await
    }

    /// Get account information
    pub async fn get_account(
        &self,
        pubkey: &Pubkey,
    ) -> Result<Option<solana_sdk::account::Account>> {
        let pubkey_str = pubkey.to_string();

        // Check cache first
        if self.config.use_cache {
            let cache = self.account_cache.read().await;
            if let Some(cached) = cache.get(&pubkey_str) {
                let age = chrono::Utc::now() - cached.timestamp;
                if age.num_seconds() < self.config.cache_ttl as i64 {
                    log::debug!("Account {} found in cache", pubkey_str);
                    return Ok(Some(cached.account.clone()));
                }
            }
        }

        // Fetch from network
        let account = tokio::task::spawn_blocking({
            let client = self.rpc_client.clone();
            let pubkey = *pubkey;
            move || client.get_account(&pubkey)
        })
        .await
        .map_err(|e| SurfDeskError::internal(format!("Task join error: {}", e)))?;

        match account {
            Ok(acc) => {
                // Update cache
                if self.config.use_cache {
                    let mut cache = self.account_cache.write().await;
                    cache.insert(
                        pubkey_str,
                        CachedAccount {
                            account: acc.clone(),
                            timestamp: chrono::Utc::now(),
                        },
                    );
                }
                log::debug!("Account {} fetched from network", pubkey.to_string());
                Ok(Some(acc))
            }
            Err(e) => {
                log::debug!("Account {} not found: {}", pubkey.to_string(), e);
                Ok(None)
            }
        }
    }

    /// Get multiple accounts for an environment
    pub async fn get_accounts(
        &self,
        _environment: &crate::types::Environment,
    ) -> Result<Vec<crate::types::Account>> {
        // For now, return empty vector - this would be implemented based on
        // the specific requirements of how accounts are associated with environments
        log::info!("Getting accounts for environment");
        Ok(vec![])
    }

    /// Get account balance in lamports
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        let balance = tokio::task::spawn_blocking({
            let client = self.rpc_client.clone();
            let pubkey = *pubkey;
            move || client.get_balance(&pubkey)
        })
        .await
        .map_err(|e| SurfDeskError::internal(format!("Task join error: {}", e)))?;

        match balance {
            Ok(bal) => {
                log::debug!("Account {} balance: {} lamports", pubkey.to_string(), bal);
                Ok(bal)
            }
            Err(e) => Err(SurfDeskError::solana_sdk(format!(
                "Failed to get balance: {}",
                e
            ))),
        }
    }

    /// Send and confirm transaction
    pub async fn send_transaction(&self, transaction: &Transaction) -> Result<Signature> {
        log::info!("Sending transaction...");

        let signature = tokio::task::spawn_blocking({
            let client = self.rpc_client.clone();
            let transaction = transaction.clone();
            move || client.send_and_confirm_transaction(&transaction)
        })
        .await
        .map_err(|e| SurfDeskError::internal(format!("Task join error: {}", e)))?;

        match signature {
            Ok(sig) => {
                log::info!("Transaction sent successfully: {}", sig);
                Ok(sig)
            }
            Err(e) => Err(SurfDeskError::solana_sdk(format!(
                "Failed to send transaction: {}",
                e
            ))),
        }
    }

    /// Simulate transaction
    pub async fn simulate_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<TransactionSimulation> {
        log::debug!("Simulating transaction...");

        let simulation = tokio::task::spawn_blocking({
            let client = self.rpc_client.clone();
            let transaction = transaction.clone();
            move || client.simulate_transaction(&transaction)
        })
        .await
        .map_err(|e| SurfDeskError::internal(format!("Task join error: {}", e)))?;

        match simulation {
            Ok(sim) => {
                let result = TransactionSimulation {
                    value: format!(
                        "Simulation completed with {} units consumed",
                        sim.value.units_consumed.unwrap_or(0)
                    ),
                    units_consumed: sim.value.units_consumed,
                    logs: sim.value.logs,
                };
                log::debug!("Transaction simulation completed");
                Ok(result)
            }
            Err(e) => Err(SurfDeskError::solana_sdk(format!(
                "Failed to simulate transaction: {}",
                e
            ))),
        }
    }

    /// Get latest blockhash
    pub async fn get_latest_blockhash(&self) -> Result<solana_sdk::hash::Hash> {
        let blockhash = tokio::task::spawn_blocking({
            let client = self.rpc_client.clone();
            move || client.get_latest_blockhash()
        })
        .await
        .map_err(|e| SurfDeskError::internal(format!("Task join error: {}", e)))?;

        match blockhash {
            Ok(hash) => {
                log::debug!("Latest blockhash: {}", hash);
                Ok(hash)
            }
            Err(e) => Err(SurfDeskError::solana_sdk(format!(
                "Failed to get latest blockhash: {}",
                e
            ))),
        }
    }

    /// Request airdrop (for devnet/testnet only)
    pub async fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> Result<Signature> {
        log::info!(
            "Requesting airdrop of {} lamports for {}",
            lamports,
            pubkey.to_string()
        );

        let signature = tokio::task::spawn_blocking({
            let client = self.rpc_client.clone();
            let pubkey = *pubkey;
            move || client.request_airdrop(&pubkey, lamports)
        })
        .await
        .map_err(|e| SurfDeskError::internal(format!("Task join error: {}", e)))?;

        match signature {
            Ok(sig) => {
                log::info!("Airdrop requested: {}", sig);
                Ok(sig)
            }
            Err(e) => Err(SurfDeskError::solana_sdk(format!(
                "Failed to request airdrop: {}",
                e
            ))),
        }
    }

    /// Clear account cache
    pub async fn clear_cache(&self) {
        let mut cache = self.account_cache.write().await;
        cache.clear();
        log::debug!("Account cache cleared");
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        let cache = self.account_cache.read().await;
        let mut total_age = chrono::Duration::zero();
        let mut oldest_entry = None;
        let mut newest_entry = None;

        for cached in cache.values() {
            let age = chrono::Utc::now() - cached.timestamp;
            total_age = total_age + age;

            if oldest_entry.is_none() || cached.timestamp < oldest_entry.unwrap() {
                oldest_entry = Some(cached.timestamp);
            }

            if newest_entry.is_none() || cached.timestamp > newest_entry.unwrap() {
                newest_entry = Some(cached.timestamp);
            }
        }

        let average_age = if cache.is_empty() {
            chrono::Duration::zero()
        } else {
            total_age / cache.len() as i32
        };

        CacheStats {
            total_entries: cache.len(),
            average_age_seconds: average_age.num_seconds() as u64,
            oldest_entry,
            newest_entry,
        }
    }

    /// Shutdown the Solana service
    pub async fn shutdown(&self) -> Result<()> {
        log::info!("Shutting down Solana service");

        // Clear cache
        self.clear_cache().await;

        // Update status
        *self.status.write().await = ConnectionStatus::Disconnected;

        Ok(())
    }
}

/// Transaction simulation result
#[derive(Debug, Clone)]
pub struct TransactionSimulation {
    /// Simulation value (placeholder for MVP)
    pub value: String, // TODO: Replace with solana_transaction_status::EncodedTransactionWithStatusMeta
    /// Units consumed
    pub units_consumed: Option<u64>,
    /// Simulation logs
    pub logs: Option<Vec<String>>,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total number of cached entries
    pub total_entries: usize,
    /// Average age of cached entries in seconds
    pub average_age_seconds: u64,
    /// Oldest entry timestamp
    pub oldest_entry: Option<chrono::DateTime<chrono::Utc>>,
    /// Newest entry timestamp
    pub newest_entry: Option<chrono::DateTime<chrono::Utc>>,
}
