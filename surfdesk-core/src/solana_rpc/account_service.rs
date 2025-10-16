//! Simple Account Service using Unified RPC Client
//! Provides account management with real Solana network integration

use crate::accounts::{Account, AccountManager};
use crate::error::{Result, SurfDeskError};
use crate::solana_rpc::{Keypair, Pubkey, RpcCommitment, Signer, SolanaNetwork, SolanaRpcClient};

/// Account service with real Solana integration
pub struct AccountService {
    rpc_client: SolanaRpcClient,
    account_manager: AccountManager,
    network: SolanaNetwork,
}

impl AccountService {
    /// Create new account service
    pub fn new(network: SolanaNetwork) -> Self {
        Self {
            rpc_client: SolanaRpcClient::new(network),
            account_manager: AccountManager::new(),
            network,
        }
    }

    /// Create account service with custom RPC URL
    pub fn new_with_url(url: &str, network: SolanaNetwork) -> Self {
        Self {
            rpc_client: SolanaRpcClient::new_with_url(url, RpcCommitment::Confirmed),
            account_manager: AccountManager::new(),
            network,
        }
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

    /// Get all accounts with real-time balances
    pub async fn get_accounts_with_balances(&mut self) -> Result<Vec<AccountWithBalance>> {
        let accounts = self.account_manager.get_accounts().to_vec();
        let mut accounts_with_balance = Vec::new();

        // Get all pubkeys for batch request
        let pubkeys: Vec<String> = accounts.iter().map(|acc| acc.pubkey.to_string()).collect();

        // Fetch balances in batch
        let account_infos = self
            .rpc_client
            .get_multiple_accounts(pubkeys.iter().map(|s| s.as_str()).collect())
            .await?;

        for (i, account) in accounts.into_iter().enumerate() {
            let balance = if let Some(Some(info)) = account_infos.get(i) {
                info.lamports
            } else {
                0
            };

            accounts_with_balance.push(AccountWithBalance {
                account,
                balance,
                balance_sol: balance as f64 / 1_000_000_000.0,
            });
        }

        Ok(accounts_with_balance)
    }

    /// Get single account with balance
    pub async fn get_account_with_balance(
        &mut self,
        pubkey: &Pubkey,
    ) -> Result<Option<AccountWithBalance>> {
        if let Some(account) = self.account_manager.get_account(pubkey) {
            let balance = self
                .rpc_client
                .get_balance(&account.pubkey.to_string())
                .await
                .unwrap_or(0);

            Ok(Some(AccountWithBalance {
                account: account.clone(),
                balance,
                balance_sol: balance as f64 / 1_000_000_000.0,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get account balance
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        self.rpc_client.get_balance(&pubkey.to_string()).await
    }

    /// Request airdrop (devnet/testnet only)
    pub async fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> Result<String> {
        let signature = self
            .rpc_client
            .request_airdrop(&pubkey.to_string(), lamports)
            .await?;
        Ok(signature.as_str().to_string())
    }

    /// Send SOL transfer
    pub async fn send_sol_transfer(
        &self,
        from_keypair: &Keypair,
        to_pubkey: &Pubkey,
        lamports: u64,
    ) -> Result<String> {
        // For MVP, create a mock transaction
        // In real implementation, this would build and sign an actual Solana transaction

        // Create mock transaction data
        let transaction_data = format!(
            "transfer_from_{}_to_{}_amount_{}",
            from_keypair.pubkey().to_string(),
            to_pubkey.to_string(),
            lamports
        );

        // Send mock transaction (base64 encoded)
        let transaction_base64 = base64::encode(transaction_data);

        let signature = self
            .rpc_client
            .send_transaction(&transaction_base64)
            .await?;
        Ok(signature.as_str().to_string())
    }

    /// Confirm transaction
    pub async fn confirm_transaction(&self, signature: &str) -> Result<bool> {
        self.rpc_client.confirm_transaction(signature).await
    }

    /// Get account info
    pub async fn get_account_info(
        &self,
        pubkey: &Pubkey,
    ) -> Result<Option<crate::solana_rpc::AccountInfo>> {
        self.rpc_client.get_account_info(&pubkey.to_string()).await
    }

    /// Get token accounts
    pub async fn get_token_accounts(&self, pubkey: &Pubkey) -> Result<Vec<serde_json::Value>> {
        self.rpc_client
            .get_token_accounts_by_owner(&pubkey.to_string())
            .await
    }

    /// Remove account
    pub fn remove_account(&mut self, pubkey: &Pubkey) -> bool {
        self.account_manager.remove_account(pubkey)
    }

    /// Get all accounts (without balance updates)
    pub fn get_accounts(&self) -> &[Account] {
        self.account_manager.get_accounts()
    }

    /// Get current network
    pub fn network(&self) -> &SolanaNetwork {
        &self.network
    }

    /// Switch network
    pub fn switch_network(&mut self, network: SolanaNetwork) {
        self.network = network.clone();
        self.rpc_client = SolanaRpcClient::new(network);
    }

    /// Test connection
    pub async fn test_connection(&self) -> Result<bool> {
        self.rpc_client.test_connection().await
    }

    /// Get RPC client reference
    pub fn rpc_client(&self) -> &SolanaRpcClient {
        &self.rpc_client
    }

    /// Get account manager reference
    pub fn account_manager(&self) -> &AccountManager {
        &self.account_manager
    }
}

/// Account with balance information
#[derive(Debug, Clone)]
pub struct AccountWithBalance {
    pub account: Account,
    pub balance: u64,
    pub balance_sol: f64,
}

impl AccountWithBalance {
    /// Create new account with balance
    pub fn new(account: Account, balance: u64) -> Self {
        Self {
            balance_sol: balance as f64 / 1_000_000_000.0,
            account,
            balance,
        }
    }

    /// Get formatted balance string
    pub fn formatted_balance(&self) -> String {
        format!("{:.9} SOL", self.balance_sol)
    }

    /// Get short pubkey display
    pub fn short_pubkey(&self) -> String {
        let pubkey_str = self.account.pubkey.to_string();
        if pubkey_str.len() > 16 {
            format!(
                "{}...{}",
                &pubkey_str[..8],
                &pubkey_str[pubkey_str.len() - 8..]
            )
        } else {
            pubkey_str
        }
    }
}

/// Transaction builder for SOL transfers
pub struct TransactionBuilder {
    from_pubkey: Pubkey,
    instructions: Vec<String>,
    signers: Vec<Keypair>,
}

impl TransactionBuilder {
    /// Create new transaction builder
    pub fn new(from_pubkey: Pubkey) -> Self {
        Self {
            from_pubkey,
            instructions: Vec::new(),
            signers: Vec::new(),
        }
    }

    /// Add SOL transfer instruction
    pub fn add_sol_transfer(mut self, to_pubkey: Pubkey, lamports: u64) -> Self {
        let instruction = format!(
            "transfer_from_{}_to_{}_amount_{}",
            self.from_pubkey.to_string(),
            to_pubkey.to_string(),
            lamports
        );
        self.instructions.push(instruction);
        self
    }

    /// Add signer
    pub fn add_signer(mut self, signer: Keypair) -> Self {
        self.signers.push(signer);
        self
    }

    /// Build and send transaction
    pub async fn build_and_send(self, rpc_client: &SolanaRpcClient) -> Result<String> {
        if self.instructions.is_empty() {
            return Err(SurfDeskError::internal("No instructions in transaction"));
        }

        // For MVP, create mock transaction data
        let transaction_data = serde_json::json!({
            "from": self.from_pubkey.to_string(),
            "instructions": self.instructions,
            "signers": self.signers.len()
        });

        // Serialize and send
        let transaction_str = serde_json::to_string(&transaction_data).map_err(|e| {
            SurfDeskError::internal(format!("Failed to serialize transaction: {}", e))
        })?;
        let transaction_base64 = base64::encode(transaction_str);

        let signature = rpc_client.send_transaction(&transaction_base64).await?;
        Ok(signature.as_str().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_service_creation() {
        let service = AccountService::new(SolanaNetwork::Devnet);
        assert_eq!(service.network(), &SolanaNetwork::Devnet);
    }

    #[test]
    fn test_account_with_balance() {
        let mut account = Account::new("Test".to_string()).unwrap().0;
        account.balance = 1_500_000_000; // 1.5 SOL

        let account_with_balance = AccountWithBalance::new(account, 1_500_000_000);
        assert_eq!(account_with_balance.formatted_balance(), "1.500000000 SOL");
        assert_eq!(account_with_balance.balance_sol, 1.5);
    }

    #[test]
    fn test_transaction_builder() {
        let from_keypair = Keypair::new();
        let to_pubkey = Pubkey::from_string("test_pubkey");

        let builder = TransactionBuilder::new(from_keypair.pubkey())
            .add_sol_transfer(to_pubkey, 1_000_000_000);

        // Should have one instruction
        assert_eq!(builder.instructions.len(), 1);
    }

    #[test]
    fn test_short_pubkey() {
        let mut account = Account::new("Test".to_string()).unwrap().0;
        let account_with_balance = AccountWithBalance::new(account, 0);

        let short = account_with_balance.short_pubkey();
        assert!(short.len() <= 19); // 8 + ... + 8
        assert!(short.contains("..."));
    }
}
