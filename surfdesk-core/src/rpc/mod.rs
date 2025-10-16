//! WASM-compatible Solana RPC client for web platform
//! Provides Solana blockchain interactions using browser's fetch API

use crate::error::{Result, SurfDeskError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// Solana RPC client for WebAssembly
pub struct SolanaRpcClient {
    rpc_url: String,
    commitment: RpcCommitment,
}

/// RPC commitment levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RpcCommitment {
    Processed,
    Confirmed,
    Finalized,
}

impl Default for RpcCommitment {
    fn default() -> Self {
        Self::Confirmed
    }
}

impl RpcCommitment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Processed => "processed",
            Self::Confirmed => "confirmed",
            Self::Finalized => "finalized",
        }
    }
}

/// Solana RPC response wrapper
#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

/// Solana RPC error
#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

/// Account information
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfo {
    pub lamports: u64,
    pub data: String,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Balance response
pub type Balance = u64;

/// Transaction signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(String);

impl Signature {
    pub fn new(sig: String) -> Self {
        Self(sig)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Signature {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Blockhash response
#[derive(Debug, Deserialize)]
pub struct Blockhash {
    pub blockhash: String,
    pub last_valid_block_height: u64,
}

impl SolanaRpcClient {
    /// Create new RPC client
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            commitment: RpcCommitment::default(),
        }
    }

    /// Create RPC client with custom commitment
    pub fn new_with_commitment(rpc_url: impl Into<String>, commitment: RpcCommitment) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            commitment,
        }
    }

    /// Make RPC request
    async fn make_request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        });

        // Create headers
        let headers = Headers::new()?;
        headers.set("Content-Type", "application/json")?;

        // Create request options
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);
        opts.headers(&headers);

        // Build URL with parameters
        let url = format!("{}?{}", self.rpc_url, serde_json::to_string(&request_body)?);

        // Create and send request
        let request = Request::new_with_str_and_init(&url, &opts)?;
        let window =
            web_sys::window().ok_or_else(|| SurfDeskError::internal("No window object"))?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| SurfDeskError::internal(format!("Failed to convert response: {:?}", e)))?;

        if !resp.ok() {
            return Err(SurfDeskError::internal(format!(
                "HTTP error: {} {}",
                resp.status(),
                resp.status_text()
            )));
        }

        // Get response text
        let text = JsFuture::from(resp.text()?).await?;
        let response_str = text
            .as_string()
            .ok_or_else(|| SurfDeskError::internal("Failed to convert response to string"))?;

        // Parse JSON response
        let rpc_response: RpcResponse<T> = serde_json::from_str(&response_str)
            .map_err(|e| SurfDeskError::internal(format!("Failed to parse RPC response: {}", e)))?;

        if let Some(error) = rpc_response.error {
            return Err(SurfDeskError::internal(format!(
                "RPC error {}: {}",
                error.code, error.message
            )));
        }

        rpc_response
            .result
            .ok_or_else(|| SurfDeskError::internal("RPC response missing result field"))
    }

    /// Get account balance
    pub async fn get_balance(&self, pubkey: &str) -> Result<Balance> {
        let params = vec![
            json!(pubkey),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        self.make_request("getBalance", params).await
    }

    /// Get account information
    pub async fn get_account(&self, pubkey: &str) -> Result<Option<AccountInfo>> {
        let params = vec![
            json!(pubkey),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        self.make_request("getAccountInfo", params).await
    }

    /// Get latest blockhash
    pub async fn get_latest_blockhash(&self) -> Result<Blockhash> {
        let params = vec![json!({ "commitment": self.commitment.as_str() })];

        self.make_request("getLatestBlockhash", params).await
    }

    /// Send transaction
    pub async fn send_transaction(&self, transaction: &str) -> Result<Signature> {
        let params = vec![
            json!(transaction),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        let signature: String = self.make_request("sendTransaction", params).await?;
        Ok(Signature::new(signature))
    }

    /// Confirm transaction
    pub async fn confirm_transaction(&self, signature: &str) -> Result<bool> {
        let params = vec![json!(signature)];

        let result: Value = self.make_request("confirmTransaction", params).await?;

        // Check if confirmation is successful
        Ok(result
            .get("value")
            .and_then(|v| v.as_bool())
            .unwrap_or(false))
    }

    /// Request airdrop (devnet/testnet only)
    pub async fn request_airdrop(&self, pubkey: &str, lamports: u64) -> Result<Signature> {
        let params = vec![json!(pubkey), json!(lamports)];

        let signature: String = self.make_request("requestAirdrop", params).await?;
        Ok(Signature::new(signature))
    }

    /// Get token accounts by owner
    pub async fn get_token_accounts_by_owner(&self, owner: &str) -> Result<Vec<Value>> {
        let params = vec![
            json!(owner),
            json!({ "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" }),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        let result: Value = self.make_request("getTokenAccountsByOwner", params).await?;

        if let Some(value) = result.get("value").and_then(|v| v.as_array()) {
            Ok(value.clone())
        } else {
            Ok(vec![])
        }
    }

    /// Get token account balance
    pub async fn get_token_account_balance(&self, account: &str) -> Result<TokenAccountBalance> {
        let params = vec![json!(account)];

        self.make_request("getTokenAccountBalance", params).await
    }

    /// Get signature statuses
    pub async fn get_signature_statuses(
        &self,
        signatures: Vec<&str>,
    ) -> Result<Vec<Option<Value>>> {
        let params = vec![
            json!(signatures),
            json!({ "searchTransactionHistory": true }),
        ];

        let result: Value = self.make_request("getSignatureStatuses", params).await?;

        if let Some(value) = result.get("value").and_then(|v| v.as_array()) {
            Ok(value.iter().cloned().collect())
        } else {
            Ok(vec![])
        }
    }
}

/// Token account balance
#[derive(Debug, Deserialize)]
pub struct TokenAccountBalance {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: Option<String>,
}

/// Simple Solana client for web
pub struct SolanaWebClient {
    rpc: SolanaRpcClient,
    network: SolanaNetwork,
}

/// Solana network types (compatible with existing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SolanaNetwork {
    Mainnet,
    #[default]
    Devnet,
    Testnet,
}

impl SolanaNetwork {
    /// Get RPC URL for network
    pub fn rpc_url(&self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.mainnet-beta.solana.com",
            Self::Devnet => "https://api.devnet.solana.com",
            Self::Testnet => "https://api.testnet.solana.com",
        }
    }

    /// Get display name for network
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet Beta",
            Self::Devnet => "Devnet",
            Self::Testnet => "Testnet",
        }
    }
}

impl SolanaWebClient {
    /// Create new client for network
    pub fn new(network: SolanaNetwork) -> Self {
        Self {
            rpc: SolanaRpcClient::new(network.rpc_url()),
            network,
        }
    }

    /// Create client with custom RPC URL
    pub fn new_with_url(url: impl Into<String>, network: SolanaNetwork) -> Self {
        Self {
            rpc: SolanaRpcClient::new(url),
            network,
        }
    }

    /// Get RPC client reference
    pub fn rpc(&self) -> &SolanaRpcClient {
        &self.rpc
    }

    /// Get current network
    pub fn network(&self) -> &SolanaNetwork {
        &self.network
    }

    /// Test connection
    pub async fn test_connection(&self) -> Result<bool> {
        match self.rpc.get_latest_blockhash().await {
            Ok(_) => Ok(true),
            Err(e) => {
                log::warn!("Failed to connect to Solana network: {}", e);
                Ok(false)
            }
        }
    }

    /// Switch network
    pub fn switch_network(&mut self, network: SolanaNetwork) {
        self.network = network;
        self.rpc = SolanaRpcClient::new(network.rpc_url());
    }
}

// Export commonly used types
pub use crate::accounts::{Account, AccountManager};
pub use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

/// WASM-compatible account service
pub struct WebAccountService {
    client: SolanaWebClient,
    account_manager: AccountManager,
}

impl WebAccountService {
    /// Create new web account service
    pub fn new(network: SolanaNetwork) -> Self {
        Self {
            client: SolanaWebClient::new(network),
            account_manager: AccountManager::new(),
        }
    }

    /// Create new account (client-side only)
    pub fn create_account(&mut self, label: String) -> Result<(Account, Keypair)> {
        let (account, keypair) = Account::new(label)
            .map_err(|e| SurfDeskError::internal(format!("Failed to create account: {}", e)))?;
        self.account_manager
            .add_account(account.clone())
            .map_err(|e| SurfDeskError::internal(format!("Failed to add account: {}", e)))?;
        Ok((account, keypair))
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

    /// Get account balance from network
    pub async fn get_balance(&self, pubkey: &str) -> Result<u64> {
        self.client.rpc.get_balance(pubkey).await
    }

    /// Request airdrop
    pub async fn request_airdrop(&self, pubkey: &str, lamports: u64) -> Result<Signature> {
        self.client.rpc.request_airdrop(pubkey, lamports).await
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
        self.client.network()
    }

    /// Switch network
    pub fn switch_network(&mut self, network: SolanaNetwork) {
        self.client.switch_network(network);
        self.account_manager.set_network(network);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_urls() {
        assert_eq!(
            SolanaNetwork::Mainnet.rpc_url(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            SolanaNetwork::Devnet.rpc_url(),
            "https://api.devnet.solana.com"
        );
        assert_eq!(
            SolanaNetwork::Testnet.rpc_url(),
            "https://api.testnet.solana.com"
        );
    }

    #[test]
    fn test_commitment_levels() {
        assert_eq!(RpcCommitment::Processed.as_str(), "processed");
        assert_eq!(RpcCommitment::Confirmed.as_str(), "confirmed");
        assert_eq!(RpcCommitment::Finalized.as_str(), "finalized");
    }

    #[test]
    fn test_web_client_creation() {
        let client = SolanaWebClient::new(SolanaNetwork::Devnet);
        assert_eq!(client.network(), &SolanaNetwork::Devnet);
    }
}
