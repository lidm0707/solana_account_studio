//! Accounts module for SurfDesk
//!
//! This module provides account management functionality for Solana accounts,
//! including account structures, validation, and management operations.

use crate::error::{Result, SurfDeskError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Solana account with its key information
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
    /// Account metadata
    pub metadata: AccountMetadata,
}

/// Additional metadata for an account
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountMetadata {
    /// Account label/name
    pub label: Option<String>,
    /// Account description
    pub description: Option<String>,
    /// Account tags for categorization
    pub tags: Vec<String>,
    /// When the account was last updated
    pub last_updated: Option<u64>,
    /// Account type classification
    pub account_type: AccountType,
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
    /// Unknown/other account type
    Unknown,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Unknown
    }
}

impl Account {
    /// Create an Account from a keypair string and label
    pub fn from_keypair(keypair_str: &str, label: String) -> Self {
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

        Account {
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
            },
        }
    }

    /// Create a new Account with a label
    pub fn new(label: String) -> Result<Self> {
        // Generate a mock pubkey for MVP implementation
        let mock_pubkey = format!(
            "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWW{}",
            (label.len() % 10)
        );

        Ok(Self {
            pubkey: mock_pubkey,
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
            },
        })
    }
}

/// Manager for handling multiple accounts
#[derive(Debug, Clone)]
pub struct AccountManager {
    /// Map of accounts by public key
    accounts: HashMap<String, Account>,
    /// Currently selected account
    selected_account: Option<String>,
}

impl AccountManager {
    /// Create a new account manager
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            selected_account: None,
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

    /// Get total balance across all accounts
    pub fn get_total_balance(&self) -> u64 {
        self.accounts.values().map(|account| account.lamports).sum()
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

        // Validate owner is also valid base58
        bs58::decode(&account.owner).into_vec().map_err(|_| {
            SurfDeskError::InvalidPubkey(format!("Invalid owner pubkey format: {}", account.owner))
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
                        .description
                        .as_ref()
                        .map(|desc| desc.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || account
                        .metadata
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
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
            },
        }
    }

    #[test]
    fn test_account_manager_new() {
        let manager = AccountManager::new();
        assert_eq!(manager.get_account_count(), 0);
        assert!(manager.get_selected_account().is_none());
    }

    #[test]
    fn test_add_account() {
        let mut manager = AccountManager::new();
        let account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        assert!(manager.add_account(account.clone()).is_ok());
        assert_eq!(manager.get_account_count(), 1);
        assert!(manager.get_account(&account.pubkey).is_some());
    }

    #[test]
    fn test_remove_account() {
        let mut manager = AccountManager::new();
        let account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        manager.add_account(account.clone()).unwrap();
        assert_eq!(manager.get_account_count(), 1);

        assert!(manager.remove_account(&account.pubkey).is_ok());
        assert_eq!(manager.get_account_count(), 0);
    }

    #[test]
    fn test_set_selected_account() {
        let mut manager = AccountManager::new();
        let account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

        manager.add_account(account.clone()).unwrap();
        assert!(manager.set_selected_account(account.pubkey.clone()).is_ok());
        assert!(manager.get_selected_account().is_some());
    }

    #[test]
    fn test_invalid_pubkey() {
        let mut manager = AccountManager::new();
        let mut account = create_test_account("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");
        account.pubkey = "invalid_pubkey".to_string();

        assert!(manager.add_account(account).is_err());
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
}
