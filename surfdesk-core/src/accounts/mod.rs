//! Solana Account Management Module
//! Provides core account functionality for MVP

use serde::{Deserialize, Serialize};

/// Create a new keypair (helper function for WASM compatibility)
pub fn create_keypair() -> String {
    // For MVP, return a mock keypair string
    // In real implementation, this would use proper WASM-compatible key generation
    "mock_keypair_base58_string".to_string()
}

/// Solana account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub pubkey: Pubkey,
    pub label: String,
    pub balance: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Account {
    /// Create a new account
    pub fn new(label: String) -> Result<(Self, String), Box<dyn std::error::Error>> {
        let keypair_str = create_keypair();
        let pubkey = Pubkey::from_string(&keypair_str);

        Ok((
            Self {
                pubkey,
                label,
                balance: 0,
                created_at: chrono::Utc::now(),
            },
            keypair_str,
        ))
    }

    /// Create account from existing keypair string
    pub fn from_keypair(keypair_str: &str, label: String) -> Self {
        let pubkey = Pubkey::from_string(keypair_str);

        Self {
            pubkey,
            label,
            balance: 0,
            created_at: chrono::Utc::now(),
        }
    }

    /// Import account from secret key
    pub fn from_secret_key(
        secret_key: &str,
        label: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // For MVP, just use the secret key as a mock keypair
        // In real implementation, this would properly decode and validate the secret key
        let pubkey = Pubkey::from_string(secret_key);

        Ok(Self {
            pubkey,
            label,
            balance: 0,
            created_at: chrono::Utc::now(),
        })
    }

    /// Export account secret key
    pub fn export_secret_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        // For MVP, return the public key as placeholder
        // In real implementation, this would return the actual secret key
        Ok(self.pubkey.to_string())
    }
}

// Re-export commonly used types from solana_rpc for web compatibility
pub use crate::solana_rpc::{Keypair, Pubkey, Signer};

/// Account manager for handling multiple accounts
pub struct AccountManager {
    accounts: Vec<Account>,
    default_network: SolanaNetwork,
}

/// Solana network types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SolanaNetwork {
    Mainnet,
    #[default]
    Devnet,
    Testnet,
}

impl AccountManager {
    /// Create new account manager
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            default_network: SolanaNetwork::Devnet,
        }
    }

    /// Add a new account
    pub fn add_account(&mut self, account: Account) -> Result<(), Box<dyn std::error::Error>> {
        self.accounts.push(account);
        Ok(())
    }

    /// Get all accounts
    pub fn get_accounts(&self) -> &[Account] {
        &self.accounts
    }

    /// Get account by pubkey
    pub fn get_account(&self, pubkey: &Pubkey) -> Option<&Account> {
        self.accounts.iter().find(|acc| acc.pubkey == *pubkey)
    }

    /// Remove account
    pub fn remove_account(&mut self, pubkey: &Pubkey) -> bool {
        let initial_len = self.accounts.len();
        self.accounts.retain(|acc| acc.pubkey != *pubkey);
        self.accounts.len() < initial_len
    }

    /// Set default network
    pub fn set_network(&mut self, network: SolanaNetwork) {
        self.default_network = network;
    }

    /// Get current network
    pub fn get_network(&self) -> &SolanaNetwork {
        &self.default_network
    }
}

impl Default for AccountManager {
    fn default() -> Self {
        Self::new()
    }
}
