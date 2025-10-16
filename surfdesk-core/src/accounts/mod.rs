//! Solana Account Management Module
//! Provides core account functionality for MVP

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

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
    pub fn new(label: String) -> Result<(Self, Keypair), Box<dyn std::error::Error>> {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();

        Ok((
            Self {
                pubkey,
                label,
                balance: 0,
                created_at: chrono::Utc::now(),
            },
            keypair,
        ))
    }

    /// Create account from existing keypair
    pub fn from_keypair(keypair: &Keypair, label: String) -> Self {
        Self {
            pubkey: keypair.pubkey(),
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
        let bytes = bs58::decode(secret_key)
            .into_vec()
            .map_err(|e| format!("Invalid secret key encoding: {}", e))?;
        let keypair =
            Keypair::from_bytes(&bytes).map_err(|e| format!("Invalid secret key: {}", e))?;
        Ok(Self::from_keypair(&keypair, label))
    }

    /// Export account secret key
    pub fn export_secret_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        // This would need access to the actual keypair
        // For MVP, we'll return the public key as placeholder
        Ok(self.pubkey.to_string())
    }
}

/// Account manager for handling multiple accounts
pub struct AccountManager {
    accounts: Vec<Account>,
    default_network: SolanaNetwork,
}

/// Solana network types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SolanaNetwork {
    Mainnet,
    Devnet,
    Testnet,
}

impl Default for SolanaNetwork {
    fn default() -> Self {
        SolanaNetwork::Devnet
    }
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
