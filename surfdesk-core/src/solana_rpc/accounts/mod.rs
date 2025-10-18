//! 🏄‍♂️ Solana Account Management Module
//!
//! This module provides comprehensive account management functionality for Solana accounts,
//! including account structures, validation, and management operations. It's designed to work
//! seamlessly with the solana_rpc module for a unified Solana interaction experience.
//!
//! ## Features
//!
//! - Account creation and management
//! - Keypair handling and validation
//! - Account metadata and categorization
//! - Balance tracking and monitoring
//! - Import/export functionality
//! - Account search and filtering

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export types from parent solana_rpc module
pub use crate::error::{Result, SurfDeskError};
pub use crate::solana_rpc::{Keypair, Pubkey, Signature, Signer, SolanaNetwork};

/// Represents a Solana account with comprehensive information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    /// The base58-encoded public key of the account
    pub pubkey: String,
    /// The lamport balance of the account
    pub lamports: u64,
    /// The data stored in the account
    pub data: Vec<u8>,
    /// The owner program of the account
    pub owner: String,
    /// Whether the account is executable
    pub executable: bool,
    /// The rent epoch for the account
    pub rent_epoch: u64,
    /// Account metadata for organization and display
    pub metadata: AccountMetadata,
}

/// Additional metadata for an account
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountMetadata {
    /// Account label/name for user identification
    pub label: Option<String>,
    /// Account description
    pub description: Option<String>,
    /// Account tags for categorization
    pub tags: Vec<String>,
    /// When the account was last updated
    pub last_updated: Option<u64>,
    /// Account type classification
    pub account_type: AccountType,
    /// Network this account belongs to
    pub network: SolanaNetwork,
    /// When the account was created/imported
    pub created_at: DateTime<Utc>,
    /// Whether this account is a favorite
    pub is_favorite: bool,
}

/// Types of accounts for categorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    /// System program account
    System,
    /// Token account
    Token,
    /// Program account
    Program,
    /// Associated token account
    AssociatedToken,
    /// Data account
    Data,
    /// Unknown/other account type
    Unknown,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Unknown
    }
}

impl Account {
    /// Create a new Account with a label and network
    pub fn new(label: String, network: SolanaNetwork) -> Result<(Self, Keypair)> {
        // Generate a new keypair
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();

        Ok((
            Self {
                pubkey,
                lamports: 0,
                data: vec![],
                owner: "11111111111111111111111111111111".to_string(), // System program
                executable: false,
                rent_epoch: 0,
                metadata: AccountMetadata {
                    label: Some(label),
                    description: None,
                    tags: vec![],
                    last_updated: None,
                    account_type: AccountType::System,
                    network,
                    created_at: Utc::now(),
                    is_favorite: false,
                },
            },
            keypair,
        ))
    }

    /// Create an Account from a keypair string and label
    pub fn from_keypair(keypair_str: &str, label: String, network: SolanaNetwork) -> Result<Self> {
        // For MVP implementation, create a basic account structure
        // In a real implementation, this would parse the actual keypair
        // and derive the public key properly

        // Generate a mock pubkey from the keypair string (simplified)
        let pubkey = if keypair_str.len() > 44 {
            // Take first 44 chars as mock pubkey for Base58 format
            keypair_str.chars().take(44).collect()
        } else {
            // Pad with zeros if too short
            format!("{:0<44}", keypair_str)
        };

        Ok(Self {
            pubkey,
            lamports: 0,
            data: vec![],
            owner: "11111111111111111111111111111111".to_string(), // System program
            executable: false,
            rent_epoch: 0,
            metadata: AccountMetadata {
                label: Some(label),
                description: None,
                tags: vec![],
                last_updated: None,
                account_type: AccountType::System,
                network,
                created_at: Utc::now(),
                is_favorite: false,
            },
        })
    }

    /// Import account from secret key
    pub fn from_secret_key(
        secret_key: &str,
        label: String,
        network: SolanaNetwork,
    ) -> Result<Self> {
        // For MVP, just use the secret key as a mock keypair
        // In real implementation, this would properly decode and validate the secret key

        // Try to derive pubkey from secret key (simplified)
        let pubkey = if secret_key.len() > 44 {
            secret_key.chars().take(44).collect()
        } else {
            format!("{:0<44}", secret_key)
        };

        Ok(Self {
            pubkey,
            lamports: 0,
            data: vec![],
            owner: "11111111111111111111111111111111".to_string(), // System program
            executable: false,
            rent_epoch: 0,
            metadata: AccountMetadata {
                label: Some(label),
                description: None,
                tags: vec![],
                last_updated: None,
                account_type: AccountType::System,
                network,
                created_at: Utc::now(),
                is_favorite: false,
            },
        })
    }

    /// Get the public key as a Pubkey object
    pub fn get_pubkey(&self) -> Result<Pubkey> {
        Ok(Pubkey::from_string(&self.pubkey))
    }

    /// Update account balance
    pub fn update_balance(&mut self, lamports: u64) {
        self.lamports = lamports;
        self.metadata.last_updated = Some(Utc::now().timestamp_micros().max(0) as u64);
    }

    /// Update account information from RPC response
    pub fn update_from_rpc(&mut self, account_info: &crate::solana_rpc::AccountInfo) {
        self.lamports = account_info.lamports;
        self.data = account_info.data.clone().into();
        self.owner = account_info.owner.clone();
        self.executable = account_info.executable;
        self.rent_epoch = account_info.rent_epoch;
        self.metadata.last_updated = Some(Utc::now().timestamp_micros().max(0) as u64);
    }

    /// Add a tag to the account
    pub fn add_tag(&mut self, tag: String) {
        if !self.metadata.tags.contains(&tag) {
            self.metadata.tags.push(tag);
        }
    }

    /// Remove a tag from the account
    pub fn remove_tag(&mut self, tag: &str) {
        self.metadata.tags.retain(|t| t != tag);
    }

    /// Toggle favorite status
    pub fn toggle_favorite(&mut self) {
        self.metadata.is_favorite = !self.metadata.is_favorite;
    }

    /// Get display name (label or pubkey)
    pub fn display_name(&self) -> &str {
        self.metadata
            .label
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(&self.pubkey)
    }

    /// Get balance in SOL (lamports to SOL conversion)
    pub fn balance_sol(&self) -> f64 {
        self.lamports as f64 / 1_000_000_000.0
    }
}

/// Manager for handling multiple accounts
#[derive(Debug, Clone, PartialEq)]
pub struct AccountManager {
    /// Map of accounts by public key
    accounts: HashMap<String, Account>,
    /// Currently selected account
    selected_account: Option<String>,
    /// Default network for new accounts
    default_network: SolanaNetwork,
}

impl AccountManager {
    /// Create a new account manager
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            selected_account: None,
            default_network: SolanaNetwork::Devnet,
        }
    }

    /// Create a new account manager with default network
    pub fn with_network(network: SolanaNetwork) -> Self {
        Self {
            accounts: HashMap::new(),
            selected_account: None,
            default_network: network,
        }
    }

    /// Add an account to the manager
    pub fn add_account(&mut self, account: Account) -> Result<()> {
        // Validate the account
        self.validate_account(&account)?;

        // Add to the accounts map
        self.accounts.insert(account.pubkey.clone(), account);
        Ok(())
    }

    /// Remove an account by public key
    pub fn remove_account(&mut self, pubkey: &str) -> Result<()> {
        if !self.accounts.contains_key(pubkey) {
            return Err(SurfDeskError::InvalidPubkey(format!(
                "Account not found: {}",
                pubkey
            )));
        }

        self.accounts.remove(pubkey);

        // Clear selected account if it was the removed one
        if let Some(selected) = &self.selected_account {
            if selected == pubkey {
                self.selected_account = None;
            }
        }

        Ok(())
    }

    /// Get an account by public key
    pub fn get_account(&self, pubkey: &str) -> Option<&Account> {
        self.accounts.get(pubkey)
    }

    /// Get all accounts
    pub fn get_all_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }

    /// Get accounts by network
    pub fn get_accounts_by_network(&self, network: SolanaNetwork) -> Vec<&Account> {
        self.accounts
            .values()
            .filter(|account| account.metadata.network == network)
            .collect()
    }

    /// Get the currently selected account
    pub fn get_selected_account(&self) -> Option<&Account> {
        if let Some(pubkey) = &self.selected_account {
            self.accounts.get(pubkey)
        } else {
            None
        }
    }

    /// Set the selected account by public key
    pub fn set_selected_account(&mut self, pubkey: String) -> Result<()> {
        if !self.accounts.contains_key(&pubkey) {
            return Err(SurfDeskError::InvalidPubkey(format!(
                "Account not found: {}",
                pubkey
            )));
        }

        self.selected_account = Some(pubkey);
        Ok(())
    }

    /// Update an account's information
    pub fn update_account(&mut self, account: Account) -> Result<()> {
        self.validate_account(&account)?;

        if !self.accounts.contains_key(&account.pubkey) {
            return Err(SurfDeskError::InvalidPubkey(format!(
                "Account not found: {}",
                account.pubkey
            )));
        }

        self.accounts.insert(account.pubkey.clone(), account);
        Ok(())
    }

    /// Get accounts by type
    pub fn get_accounts_by_type(&self, account_type: AccountType) -> Vec<&Account> {
        self.accounts
            .values()
            .filter(|account| account.metadata.account_type == account_type)
            .collect()
    }

    /// Get favorite accounts
    pub fn get_favorite_accounts(&self) -> Vec<&Account> {
        self.accounts
            .values()
            .filter(|account| account.metadata.is_favorite)
            .collect()
    }

    /// Get total balance across all accounts
    pub fn get_total_balance(&self) -> u64 {
        self.accounts.values().map(|account| account.lamports).sum()
    }

    /// Get total balance in SOL
    pub fn get_total_balance_sol(&self) -> f64 {
        self.get_total_balance() as f64 / 1_000_000_000.0
    }

    /// Get account count
    pub fn get_account_count(&self) -> usize {
        self.accounts.len()
    }

    /// Clear all accounts
    pub fn clear_all(&mut self) {
        self.accounts.clear();
        self.selected_account = None;
    }

    /// Validate an account's public key
    fn validate_account(&self, account: &Account) -> Result<()> {
        // Basic validation - check if pubkey is valid base58
        if account.pubkey.is_empty() {
            return Err(SurfDeskError::InvalidPubkey(
                "Public key cannot be empty".to_string(),
            ));
        }

        // Try to decode as base58 to validate format
        bs58::decode(&account.pubkey).into_vec().map_err(|_| {
            SurfDeskError::InvalidPubkey(format!("Invalid base58 format: {}", account.pubkey))
        })?;

        Ok(())
    }

    /// Search accounts by label or pubkey
    pub fn search_accounts(&self, query: &str) -> Vec<&Account> {
        let query_lower = query.to_lowercase();
        self.accounts
            .values()
            .filter(|account| {
                account.pubkey.to_lowercase().contains(&query_lower)
                    || account
                        .metadata
                        .label
                        .as_ref()
                        .map(|label| label.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || account
                        .metadata
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get accounts with a specific tag
    pub fn get_accounts_by_tag(&self, tag: &str) -> Vec<&Account> {
        self.accounts
            .values()
            .filter(|account| account.metadata.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Set default network
    pub fn set_default_network(&mut self, network: SolanaNetwork) {
        self.default_network = network;
    }

    /// Get default network
    pub fn get_default_network(&self) -> SolanaNetwork {
        self.default_network
    }

    /// Export accounts to JSON
    pub fn export_accounts(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.accounts.values().collect::<Vec<_>>())
            .map_err(|e| SurfDeskError::Serialization(format!("Failed to export accounts: {}", e)))
    }

    /// Import accounts from JSON
    pub fn import_accounts(&mut self, json: &str) -> Result<usize> {
        let imported_accounts: Vec<Account> = serde_json::from_str(json).map_err(|e| {
            SurfDeskError::Serialization(format!("Failed to import accounts: {}", e))
        })?;

        let mut count = 0;
        for account in imported_accounts {
            if self.validate_account(&account).is_ok() {
                self.accounts.insert(account.pubkey.clone(), account);
                count += 1;
            }
        }

        Ok(count)
    }
}

impl Default for AccountManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_account(pubkey: &str) -> Account {
        Account {
            pubkey: pubkey.to_string(),
            lamports: 1000000,
            data: vec![],
            owner: "11111111111111111111111111111111".to_string(),
            executable: false,
            rent_epoch: 0,
            metadata: AccountMetadata {
                label: Some("Test Account".to_string()),
                description: None,
                tags: vec!["test".to_string()],
                last_updated: None,
                account_type: AccountType::System,
                network: SolanaNetwork::Devnet,
                created_at: Utc::now(),
                is_favorite: false,
            },
        }
    }

    #[test]
    fn test_account_manager_new() {
        let manager = AccountManager::new();
        assert_eq!(manager.get_account_count(), 0);
        assert!(manager.get_selected_account().is_none());
        assert_eq!(manager.get_default_network(), SolanaNetwork::Devnet);
    }

    #[test]
    fn test_add_account() {
        let mut manager = AccountManager::new();
        let account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        manager.add_account(account).unwrap();
        assert_eq!(manager.get_account_count(), 1);
    }

    #[test]
    fn test_remove_account() {
        let mut manager = AccountManager::new();
        let account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        manager.add_account(account).unwrap();
        assert_eq!(manager.get_account_count(), 1);

        manager
            .remove_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM")
            .unwrap();
        assert_eq!(manager.get_account_count(), 0);
    }

    #[test]
    fn test_set_selected_account() {
        let mut manager = AccountManager::new();
        let account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        manager.add_account(account).unwrap();
        manager
            .set_selected_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string())
            .unwrap();

        let selected = manager.get_selected_account().unwrap();
        assert_eq!(
            selected.pubkey,
            "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"
        );
    }

    #[test]
    fn test_invalid_pubkey() {
        let mut manager = AccountManager::new();
        let account = create_test_account(""); // Empty pubkey

        let result = manager.add_account(account);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_accounts() {
        let mut manager = AccountManager::new();
        let account1 = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");
        let account2 = create_test_account("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

        manager.add_account(account1).unwrap();
        manager.add_account(account2).unwrap();

        // Search by pubkey
        let results = manager.search_accounts("9WzDX");
        assert_eq!(results.len(), 1);

        // Search by label
        let results = manager.search_accounts("test");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_account_new() {
        let result = Account::new("Test Account".to_string(), SolanaNetwork::Devnet);
        assert!(result.is_ok());

        let (account, _keypair) = result.unwrap();
        assert_eq!(account.metadata.label, Some("Test Account".to_string()));
        assert_eq!(account.metadata.network, SolanaNetwork::Devnet);
        assert_eq!(account.balance_sol(), 0.0);
    }

    #[test]
    fn test_account_from_secret_key() {
        let result = Account::from_secret_key(
            "test_secret_key_123456789",
            "Imported Account".to_string(),
            SolanaNetwork::Mainnet,
        );

        assert!(result.is_ok());
        let account = result.unwrap();
        assert_eq!(account.metadata.label, Some("Imported Account".to_string()));
        assert_eq!(account.metadata.network, SolanaNetwork::Mainnet);
    }

    #[test]
    fn test_account_tags() {
        let mut account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        account.add_tag("main".to_string());
        assert!(account.metadata.tags.contains(&"main".to_string()));

        account.remove_tag("main");
        assert!(!account.metadata.tags.contains(&"main".to_string()));
    }

    #[test]
    fn test_favorite_accounts() {
        let mut manager = AccountManager::new();
        let mut account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        account.toggle_favorite();
        manager.add_account(account).unwrap();

        let favorites = manager.get_favorite_accounts();
        assert_eq!(favorites.len(), 1);
    }

    #[test]
    fn test_total_balance() {
        let mut manager = AccountManager::new();
        let account1 = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");
        let mut account2 = create_test_account("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
        account2.lamports = 2000000;

        manager.add_account(account1).unwrap();
        manager.add_account(account2).unwrap();

        assert_eq!(manager.get_total_balance(), 3000000);
        assert_eq!(manager.get_total_balance_sol(), 0.003);
    }
}
