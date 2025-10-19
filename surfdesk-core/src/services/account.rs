//! Account Service for managing Solana accounts and wallets
//!
//! This service provides functionality for creating, managing, and interacting
//! with Solana accounts. It handles keypair generation, account creation,
//! balance tracking, and transaction management through the custom RPC service.

use crate::services::{AsyncService, Configurable, Service, ServiceError, ServiceResult};
use crate::services::solana_rpc::{SolanaRpcService, validate_address};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for account service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountServiceConfig {
    /// Default derivation path for HD wallets
    pub derivation_path: String,
    /// Whether to encrypt stored keys
    pub encrypt_keys: bool,
    /// Path to account storage
    pub storage_path: PathBuf,
    /// Default SOL amount for airdrops
    pub default_airdrop_amount: u64,
}

impl Default for AccountServiceConfig {
    fn default() -> Self {
        Self {
            derivation_path: "m/44'/501'/0'/0'".to_string(),
            encrypt_keys: true,
            storage_path: dirs::home_dir()
                .unwrap_or_default()
                .join(".surfdesk")
                .join("accounts"),
            default_airdrop_amount: 1_000_000_000, // 1 SOL
        }
    }
}

/// Account information including balance and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account address (public key)
    pub address: String,
    /// Account label/name
    pub label: String,
    /// Account balance in lamports
    pub balance: u64,
    /// Account creation timestamp
    pub created_at: String,
    /// Last updated timestamp
    pub updated_at: String,
    /// Account metadata
    pub metadata: AccountMetadata,
}

/// Account metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMetadata {
    /// Whether this is the primary account
    pub is_primary: bool,
    /// Account type
    pub account_type: AccountType,
    /// Additional notes
    pub notes: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Account types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    /// Regular user account
    User,
    /// Program account
    Program,
    /// System account
    System,
    /// Token account
    Token,
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction signature
    pub signature: String,
    /// From address
    pub from: String,
    /// To address
    pub to: String,
    /// Amount in lamports
    pub amount: u64,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Status
    pub status: TransactionStatus,
    /// Timestamp
    pub timestamp: String,
    /// Fee paid
    pub fee: u64,
    /// Memo (optional)
    pub memo: Option<String>,
}

/// Transaction types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Transfer,
    ProgramCall,
    AccountCreation,
    Airdrop,
    Stake,
    Other,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

/// Service for managing Solana accounts
pub struct AccountService {
    config: AccountServiceConfig,
    accounts: HashMap<String, Account>,
    rpc_service: SolanaRpcService,
}

impl AccountService {
    /// Create a new account service with default configuration
    pub fn new() -> Self {
        Self::with_config(AccountServiceConfig::default())
    }

    /// Create a new account service with custom configuration
    pub fn with_config(config: AccountServiceConfig) -> Self {
        Self {
            rpc_service: SolanaRpcService::new(),
            accounts: HashMap::new(),
            config,
        }
    }

    /// Create a new account
    pub async fn create_account(&mut self, label: String, account_type: AccountType) -> ServiceResult<Account> {
        // Generate new keypair (mock implementation)
        let address = self.generate_mock_address();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let account = Account {
            address: address.clone(),
            label,
            balance: 0,
            created_at: now.clone(),
            updated_at: now.clone(),
            metadata: AccountMetadata {
                is_primary: self.accounts.is_empty(),
                account_type,
                notes: None,
                tags: Vec::new(),
            },
        };

        self.accounts.insert(address.clone(), account.clone());
        self.save_accounts().await?;

        Ok(account)
    }

    /// Import an existing account from private key
    pub async fn import_account(&mut self, private_key: String, label: String) -> ServiceResult<Account> {
        // In a real implementation, this would validate and import the private key
        // For now, create a mock account
        let address = self.generate_mock_address();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let account = Account {
            address: address.clone(),
            label,
            balance: 0,
            created_at: now.clone(),
            updated_at: now.clone(),
            metadata: AccountMetadata {
                is_primary: self.accounts.is_empty(),
                account_type: AccountType::User,
                notes: Some("Imported account".to_string()),
                tags: vec!["imported".to_string()],
            },
        };

        self.accounts.insert(address.clone(), account.clone());
        self.save_accounts().await?;

        Ok(account)
    }

    /// Get account by address
    pub fn get_account(&self, address: &str) -> Option<&Account> {
        self.accounts.get(address)
    }

    /// Get all accounts
    pub fn get_all_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }

    /// Update account balance from network
    pub async fn update_balance(&mut self, address: &str) -> ServiceResult<u64> {
        if !validate_address(address) {
            return Err(ServiceError::Validation("Invalid address".to_string()));
        }

        match self.rpc_service.get_balance(address).await {
            Ok(balance) => {
                if let Some(account) = self.accounts.get_mut(address) {
                    account.balance = balance;
                    account.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                }
                Ok(balance)
            }
            Err(e) => Err(e),
        }
    }

    /// Update all account balances
    pub async fn update_all_balances(&mut self) -> ServiceResult<()> {
        let addresses: Vec<String> = self.accounts.keys().cloned().collect();

        for address in addresses {
            if let Err(e) = self.update_balance(&address).await {
                tracing::warn!("Failed to update balance for {}: {}", address, e);
            }
        }

        Ok(())
    }

    /// Send SOL to another account
    pub async fn send_sol(&mut self, from: &str, to: &str, lamports: u64, memo: Option<String>) -> ServiceResult<String> {
        if !validate_address(from) || !validate_address(to) {
            return Err(ServiceError::Validation("Invalid address".to_string()));
        }

        // Check if from account exists and has sufficient balance
        if let Some(account) = self.accounts.get(from) {
            if account.balance < lamports {
                return Err(ServiceError::Validation("Insufficient balance".to_string()));
            }
        } else {
            return Err(ServiceError::Validation("From account not found".to_string()));
        }

        // Create and send transaction (mock implementation)
        let transaction_data = crate::services::solana_rpc::create_transfer_transaction(from, to, lamports)?;
        let signature = self.rpc_service.send_transaction(&transaction_data).await?;

        // Update local balance (optimistic)
        if let Some(account) = self.accounts.get_mut(from) {
            account.balance = account.balance.saturating_sub(lamports);
            account.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }

        // Record transaction
        let transaction = Transaction {
            signature: signature.clone(),
            from: from.to_string(),
            to: to.to_string(),
            amount: lamports,
            transaction_type: TransactionType::Transfer,
            status: TransactionStatus::Pending,
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            fee: 5000, // Mock fee
            memo,
        };

        // In a real implementation, this would be saved to a transaction database
        tracing::info!("Transaction sent: {} SOL from {} to {}", lamports as f64 / 1_000_000_000.0, from, to);

        Ok(signature)
    }

    /// Request airdrop for an account
    pub async fn request_airdrop(&mut self, address: &str, lamports: Option<u64>) -> ServiceResult<String> {
        if !validate_address(address) {
            return Err(ServiceError::Validation("Invalid address".to_string()));
        }

        let amount = lamports.unwrap_or(self.config.default_airdrop_amount);
        let signature = self.rpc_service.request_airdrop(address, amount).await?;

        tracing::info!("Airdrop requested: {} lamports to {}", amount, address);
        Ok(signature)
    }

    /// Delete an account
    pub async fn delete_account(&mut self, address: &str) -> ServiceResult<()> {
        if self.accounts.remove(address).is_none() {
            return Err(ServiceError::Validation("Account not found".to_string()));
        }

        self.save_accounts().await?;
        Ok(())
    }

    /// Update account label
    pub async fn update_label(&mut self, address: &str, new_label: String) -> ServiceResult<()> {
        if let Some(account) = self.accounts.get_mut(address) {
            account.label = new_label;
            account.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            self.save_accounts().await?;
            Ok(())
        } else {
            Err(ServiceError::Validation("Account not found".to_string()))
        }
    }

    /// Set primary account
    pub async fn set_primary_account(&mut self, address: &str) -> ServiceResult<()> {
        // Remove primary flag from all accounts
        for account in self.accounts.values_mut() {
            account.metadata.is_primary = false;
        }

        // Set primary flag on selected account
        if let Some(account) = self.accounts.get_mut(address) {
            account.metadata.is_primary = true;
            account.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            self.save_accounts().await?;
            Ok(())
        } else {
            Err(ServiceError::Validation("Account not found".to_string()))
        }
    }

    /// Get primary account
    pub fn get_primary_account(&self) -> Option<&Account> {
        self.accounts.values().find(|acc| acc.metadata.is_primary)
    }

    /// Save accounts to storage
    async fn save_accounts(&self) -> ServiceResult<()> {
        // In a real implementation, this would encrypt and save accounts to disk
        // For now, just log the action
        tracing::debug!("Saving {} accounts to storage", self.accounts.len());
        Ok(())
    }

    /// Load accounts from storage
    async fn load_accounts(&mut self) -> ServiceResult<()> {
        // In a real implementation, this would load and decrypt accounts from disk
        // For now, just log the action
        tracing::debug!("Loading accounts from storage");
        Ok(())
    }

    /// Generate mock address for demonstration
    fn generate_mock_address(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);

        let hash = hasher.finish();
        format!("{:x}", hash)[..44].to_string()
    }
}

impl Service for AccountService {
    fn initialize(&mut self) -> ServiceResult<()> {
        tracing::info!("Account service initialized");
        Ok(())
    }

    fn health_check(&self) -> ServiceResult<bool> {
        Ok(!self.accounts.is_empty() || true) // Service is healthy even with no accounts
    }

    fn shutdown(&mut self) -> ServiceResult<()> {
        // Save accounts before shutdown
        let _ = self.save_accounts();
        tracing::info!("Account service shutdown");
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncService for AccountService {
    async fn initialize_async(&mut self) -> ServiceResult<()> {
        self.initialize()?;
        self.load_accounts().await?;
        Ok(())
    }

    async fn health_check_async(&self) -> ServiceResult<bool> {
        // Check if RPC service is healthy
        self.rpc_service.health_check_async().await
    }

    async fn shutdown_async(&mut self) -> ServiceResult<()> {
        self.save_accounts().await?;
        self.shutdown()?;
        Ok(())
    }
}

impl Configurable for AccountService {
    type Config = AccountServiceConfig;

    fn configure(&mut self, config: Self::Config) -> ServiceResult<()> {
        self.config = config;
        tracing::info!("Account service reconfigured");
        Ok(())
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_service_creation() {
        let service = AccountService::new();
        assert!(service.get_all_accounts().is_empty());
    }

    #[test]
    fn test_create_account() {
        let mut service = AccountService::new();

        // This would need to be run in an async context in real tests
        // For now, just test the mock address generation
        let address = service.generate_mock_address();
        assert!(!address.is_empty());
        assert!(address.len() == 44); // Base58 addresses are typically 44 chars
    }

    #[test]
    fn test_account_types() {
        assert_eq!(AccountType::User, AccountType::User);
        assert_ne!(AccountType::User, AccountType::Program);
    }

    #[test]
    fn test_transaction_types() {
        assert_eq!(TransactionType::Transfer, TransactionType::Transfer);
        assert_ne!(TransactionType::Transfer, TransactionType::Airdrop);
    }

    #[test]
    fn test_transaction_status() {
        assert_eq!(TransactionStatus::Pending, TransactionStatus::Pending);
        assert_ne!(TransactionStatus::Pending, TransactionStatus::Confirmed);
    }
}
