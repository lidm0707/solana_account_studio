//! # Solana Service Module
//!
//! This module provides Solana blockchain integration for the SurfDesk application.
//! It handles RPC connections, account management, transaction operations, and
//! program interactions across all platforms using our unified RPC client.

use crate::accounts::{Account, AccountManager};
use crate::error::{Result, SurfDeskError};
use crate::solana_rpc::{Keypair, Pubkey, Signature, SolanaNetwork, SolanaRpcClient};

/// Enhanced Solana service for blockchain interactions
pub struct SolanaService {
    rpc_client: SolanaRpcClient,
}

impl SolanaService {
    /// Create new Solana service
    pub async fn new(rpc_url: String) -> Result<Self> {
        let rpc_client =
            SolanaRpcClient::new_with_url(&rpc_url, crate::solana_rpc::RpcCommitment::Confirmed);

        Ok(Self { rpc_client })
    }

    /// Get account balance
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        self.rpc_client.get_balance(&pubkey.to_string()).await
    }

    /// Get account information
    pub async fn get_account_info(
        &self,
        pubkey: &Pubkey,
    ) -> Result<Option<crate::solana_rpc::AccountInfo>> {
        self.rpc_client.get_account_info(&pubkey.to_string()).await
    }

    /// Send transaction
    pub async fn send_transaction(&self, transaction: &str) -> Result<Signature> {
        let signature = self.rpc_client.send_transaction(transaction).await?;
        Ok(Signature::new(signature.as_str().to_string()))
    }

    /// Confirm transaction
    pub async fn confirm_transaction(&self, signature: &Signature) -> Result<bool> {
        self.rpc_client
            .confirm_transaction(signature.as_str())
            .await
    }

    /// Get transaction status
    pub async fn get_transaction_status(&self, signature: &Signature) -> Result<bool> {
        self.confirm_transaction(signature).await
    }

    /// Get latest blockhash
    pub async fn get_latest_blockhash(&self) -> Result<crate::solana_rpc::LatestBlockhash> {
        self.rpc_client.get_latest_blockhash().await
    }

    /// Request airdrop (devnet/testnet only)
    pub async fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> Result<Signature> {
        let signature = self
            .rpc_client
            .request_airdrop(&pubkey.to_string(), lamports)
            .await?;
        Ok(Signature::new(signature.as_str().to_string()))
    }

    /// Get token accounts by owner
    pub async fn get_token_accounts(&self, pubkey: &Pubkey) -> Result<Vec<serde_json::Value>> {
        self.rpc_client
            .get_token_accounts_by_owner(&pubkey.to_string())
            .await
    }

    /// Get token account balance
    pub async fn get_token_account_balance(
        &self,
        account: &str,
    ) -> Result<crate::solana_rpc::TokenAccountBalance> {
        self.rpc_client.get_token_account_balance(account).await
    }

    /// Get current RPC URL
    pub fn rpc_url(&self) -> String {
        self.rpc_client.rpc_url().to_string()
    }

    /// Test connection to RPC endpoint
    pub async fn test_connection(&self) -> Result<bool> {
        self.rpc_client.test_connection().await
    }

    /// Get current network
    pub fn network(&self) -> &SolanaNetwork {
        // We'll determine network from RPC URL
        let url = self.rpc_url();
        if url.contains("mainnet") {
            &SolanaNetwork::Mainnet
        } else if url.contains("testnet") {
            &SolanaNetwork::Testnet
        } else {
            &SolanaNetwork::Devnet
        }
    }

    /// Shutdown the service
    pub fn shutdown(&self) -> Result<()> {
        log::info!("SolanaService shutdown complete");
        Ok(())
    }
}

impl std::fmt::Debug for SolanaService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolanaService")
            .field("rpc_url", &self.rpc_url())
            .field("network", self.network())
            .finish()
    }
}

/// WebAccountService for simplified account management
pub struct WebAccountService {
    solana_service: SolanaService,
    account_manager: AccountManager,
}

impl WebAccountService {
    /// Create new web account service
    pub async fn new(network: SolanaNetwork) -> Result<Self> {
        let solana_service = SolanaService::new(network.rpc_url().to_string()).await?;
        let account_manager = AccountManager::new();

        Ok(Self {
            solana_service,
            account_manager,
        })
    }

    /// Create new account
    pub fn create_account(&mut self, label: String) -> Result<(Account, Keypair)> {
        let (account, keypair) = Account::new(label)
            .map_err(|e| SurfDeskError::internal(format!("Failed to create account: {}", e)))?;

        self.account_manager
            .add_account(account.clone())
            .map_err(|e| SurfDeskError::internal(format!("Failed to add account: {}", e)))?;

        Ok((account, Keypair::with_secret(keypair)))
    }

    /// Import account from secret key
    pub fn import_account(&mut self, secret_key: &str, label: String) -> Result<Account> {
        let account = Account::from_secret_key(secret_key, label)
            .map_err(|e| SurfDeskError::internal(format!("Failed to import account: {}", e)))?;

        self.account_manager
            .add_account(account.clone())
            .map_err(|e| SurfDeskError::internal(format!("Failed to add account: {}", e)))?;

        Ok(account)
    }

    /// Get account balance
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        self.solana_service.get_balance(pubkey).await
    }

    /// Request airdrop
    pub async fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> Result<String> {
        let signature = self
            .solana_service
            .request_airdrop(pubkey, lamports)
            .await?;
        Ok(signature.to_string())
    }

    /// Send transaction
    pub async fn send_transaction(&self, transaction: &str) -> Result<String> {
        let signature = self.solana_service.send_transaction(transaction).await?;
        Ok(signature.to_string())
    }

    /// Get all accounts
    pub fn get_accounts(&self) -> &[Account] {
        self.account_manager.get_accounts()
    }

    /// Remove account
    pub fn remove_account(&mut self, pubkey: &Pubkey) -> bool {
        self.account_manager.remove_account(pubkey)
    }

    /// Get current network
    pub fn network(&self) -> &SolanaNetwork {
        self.solana_service.network()
    }

    /// Test connection
    pub async fn test_connection(&self) -> Result<bool> {
        self.solana_service.test_connection().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_service_creation() {
        // Test would require async runtime
        // For now, just verify the struct compiles
        let _service = SolanaService {
            rpc_client: SolanaRpcClient::new(SolanaNetwork::Devnet),
        };
    }

    #[test]
    fn test_solana_network_display() {
        assert_eq!(SolanaNetwork::Devnet.display_name(), "Devnet");
        assert_eq!(SolanaNetwork::Mainnet.display_name(), "Mainnet Beta");
        assert_eq!(SolanaNetwork::Testnet.display_name(), "Testnet");
    }

    #[test]
    fn test_solana_network_rpc_url() {
        assert_eq!(
            SolanaNetwork::Devnet.rpc_url(),
            "https://api.devnet.solana.com"
        );
        assert_eq!(
            SolanaNetwork::Mainnet.rpc_url(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            SolanaNetwork::Testnet.rpc_url(),
            "https://api.testnet.solana.com"
        );
    }
}
