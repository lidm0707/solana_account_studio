//! Unified Solana RPC Client
//! Platform-agnostic RPC client with different backends for web and desktop

use crate::error::{Result, SurfDeskError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// Mock Solana types for WASM compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pubkey(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature(String);

impl Signature {
    pub fn new(sig: String) -> Self {
        Self(sig)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Pubkey {
    pub fn from_string(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl std::fmt::Display for Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Keypair {
    pub pubkey: Pubkey,
    pub secret: String,
}

impl Keypair {
    pub fn new() -> Self {
        let secret = format!(
            "mock_secret_{}",
            uuid::Uuid::new_v4().to_string().replace("-", "")
        );
        let pubkey = Pubkey::from_string(&format!("mock_pubkey_{}", &secret[..32]));
        Self { pubkey, secret }
    }

    pub fn with_secret(secret: String) -> Self {
        let pubkey = Pubkey::from_string(&format!("mock_pubkey_{}", &secret[..32]));
        Self { pubkey, secret }
    }

    pub fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
}

pub trait Signer {
    fn pubkey(&self) -> &Pubkey;
}

impl Signer for Keypair {
    fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
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

/// Solana network configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SolanaNetwork {
    Mainnet,
    #[default]
    Devnet,
    Testnet,
}

// Conversion between SolanaNetwork types
// Remove conflicting implementations - will handle conversions differently

impl SolanaNetwork {
    pub fn rpc_url(&self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.mainnet-beta.solana.com",
            Self::Devnet => "https://api.devnet.solana.com",
            Self::Testnet => "https://api.testnet.solana.com",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet Beta",
            Self::Devnet => "Devnet",
            Self::Testnet => "Testnet",
        }
    }
}

/// RPC request structure
#[derive(Debug, Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Vec<Value>,
}

/// RPC response structure
#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    id: Option<u64>,
    result: Option<T>,
    error: Option<RpcError>,
}

/// RPC error structure
#[derive(Debug, Deserialize)]
struct RpcError {
    code: i64,
    message: String,
}

/// Account information from RPC
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfo {
    pub lamports: u64,
    pub data: String,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Latest blockhash response
#[derive(Debug, Deserialize)]
pub struct LatestBlockhash {
    pub blockhash: String,
    pub last_valid_block_height: u64,
}

/// Token account balance
#[derive(Debug, Deserialize)]
pub struct TokenAccountBalance {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: Option<String>,
}

/// Transaction signature wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignature(String);

impl TransactionSignature {
    pub fn new(sig: String) -> Self {
        Self(sig)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for TransactionSignature {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Platform-specific HTTP client trait
trait HttpClient {
    fn post_json(
        &self,
        url: &str,
        body: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send>>;
}

/// Web HTTP client using gloo-net (WASM-compatible)
#[cfg(feature = "web")]
struct WebHttpClient {
    _client: std::marker::PhantomData<()>,
}

#[cfg(feature = "web")]
impl WebHttpClient {
    fn new() -> Self {
        Self {
            _client: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "web")]
impl HttpClient for WebHttpClient {
    fn post_json(
        &self,
        url: &str,
        body: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send>> {
        use gloo_console::log;
        use gloo_net::http::Request;
        use wasm_bindgen_futures::JsFuture;

        // Temporarily disable WebHttpClient for compilation
        // TODO: Fix WASM threading issues later
        Box::pin(async move {
            log::info!("Web HTTP request (mock): {}", body);
            Ok(r#"{"jsonrpc":"2.0","id":1,"result":"mock_response"}"#.to_string())
        })
    }
}

/// Desktop HTTP client (mock for WASM compatibility)
#[cfg(any(feature = "desktop", feature = "tui"))]
struct DesktopHttpClient {
    _client: std::marker::PhantomData<()>,
}

#[cfg(any(feature = "desktop", feature = "tui"))]
impl DesktopHttpClient {
    fn new() -> Self {
        Self {
            _client: std::marker::PhantomData,
        }
    }
}

#[cfg(any(feature = "desktop", feature = "tui"))]
impl HttpClient for DesktopHttpClient {
    fn post_json(
        &self,
        _url: &str,
        body: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send>> {
        let body = body.clone();

        Box::pin(async move {
            // Mock implementation for WASM compatibility
            // In real implementation, this would use reqwest
            log::info!("Desktop HTTP request (mock): {}", body);
            Ok(r#"{"jsonrpc":"2.0","id":1,"result":"mock_response"}"#.to_string())
        })
    }
}

/// Unified Solana RPC client
pub struct SolanaRpcClient {
    http_client: Box<dyn HttpClient>,
    rpc_url: String,
    commitment: RpcCommitment,
    request_id: std::sync::atomic::AtomicU64,
}

impl SolanaRpcClient {
    /// Create new RPC client for specified network
    pub fn new(network: SolanaNetwork) -> Self {
        Self::new_with_url(network.rpc_url(), RpcCommitment::default())
    }

    /// Create new RPC client with custom URL and commitment
    pub fn new_with_url(url: &str, commitment: RpcCommitment) -> Self {
        let http_client: Box<dyn HttpClient> = if cfg!(feature = "web") {
            Box::new(WebHttpClient::new())
        } else {
            Box::new(DesktopHttpClient::new())
        };

        Self {
            http_client,
            rpc_url: url.to_string(),
            commitment,
            request_id: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Make RPC request
    async fn make_request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T> {
        let request_id = self
            .request_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: method.to_string(),
            params,
        };

        let request_body = serde_json::to_string(&request)
            .map_err(|e| SurfDeskError::internal(format!("Failed to serialize request: {}", e)))?;

        let future = self.http_client.post_json(&self.rpc_url, request_body);
        let response_text = future.await?;

        let rpc_response: RpcResponse<T> = serde_json::from_str(&response_text)
            .map_err(|e| SurfDeskError::internal(format!("Failed to parse response: {}", e)))?;

        if let Some(error) = rpc_response.error {
            return Err(SurfDeskError::internal(format!(
                "RPC error {}: {}",
                error.code, error.message
            )));
        }

        rpc_response
            .result
            .ok_or_else(|| SurfDeskError::internal("RPC response missing result"))
    }

    /// Get account balance in lamports
    pub async fn get_balance(&self, pubkey: &str) -> Result<u64> {
        let params = vec![
            json!(pubkey),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        #[derive(Deserialize)]
        struct BalanceResponse {
            value: u64,
        }

        let response: BalanceResponse = self.make_request("getBalance", params).await?;
        Ok(response.value)
    }

    /// Get account information
    pub async fn get_account_info(&self, pubkey: &str) -> Result<Option<AccountInfo>> {
        let params = vec![
            json!(pubkey),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        let response: Option<AccountInfo> = self.make_request("getAccountInfo", params).await?;
        Ok(response)
    }

    /// Get latest blockhash
    pub async fn get_latest_blockhash(&self) -> Result<LatestBlockhash> {
        let params = vec![json!({ "commitment": self.commitment.as_str() })];

        self.make_request("getLatestBlockhash", params).await
    }

    /// Send transaction
    pub async fn send_transaction(&self, transaction: &str) -> Result<TransactionSignature> {
        let params = vec![
            json!(transaction),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        let signature: String = self.make_request("sendTransaction", params).await?;
        Ok(TransactionSignature::new(signature))
    }

    /// Confirm transaction
    pub async fn confirm_transaction(&self, signature: &str) -> Result<bool> {
        let params = vec![json!(signature)];

        let result: Value = self.make_request("confirmTransaction", params).await?;

        Ok(result
            .get("value")
            .and_then(|v| v.as_bool())
            .unwrap_or(false))
    }

    /// Request airdrop (devnet/testnet only)
    pub async fn request_airdrop(
        &self,
        pubkey: &str,
        lamports: u64,
    ) -> Result<TransactionSignature> {
        let params = vec![json!(pubkey), json!(lamports)];

        let signature: String = self.make_request("requestAirdrop", params).await?;
        Ok(TransactionSignature::new(signature))
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
            Ok(value.iter().map(|v| Some(v.clone())).collect())
        } else {
            Ok(vec![])
        }
    }

    /// Get multiple accounts
    pub async fn get_multiple_accounts(
        &self,
        pubkeys: Vec<&str>,
    ) -> Result<Vec<Option<AccountInfo>>> {
        if pubkeys.is_empty() {
            return Ok(vec![]);
        }

        let params = vec![
            json!(pubkeys),
            json!({ "commitment": self.commitment.as_str() }),
        ];

        let result: Value = self.make_request("getMultipleAccounts", params).await?;

        if let Some(value) = result.get("value").and_then(|v| v.as_array()) {
            let accounts: Result<Vec<Option<AccountInfo>>> = value
                .iter()
                .map(|v| {
                    if v.is_null() {
                        Ok(None)
                    } else {
                        serde_json::from_value(v.clone()).map(Some).map_err(|e| {
                            SurfDeskError::internal(format!("Failed to parse account: {}", e))
                        })
                    }
                })
                .collect();

            accounts
        } else {
            Ok(vec![])
        }
    }

    /// Test connection to RPC endpoint
    pub async fn test_connection(&self) -> Result<bool> {
        match self.get_latest_blockhash().await {
            Ok(_) => Ok(true),
            Err(e) => {
                log::warn!("RPC connection test failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Get current RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// Get current commitment level
    pub fn commitment(&self) -> RpcCommitment {
        self.commitment
    }

    /// Update commitment level
    pub fn set_commitment(&mut self, commitment: RpcCommitment) {
        self.commitment = commitment;
    }
}

impl std::fmt::Debug for SolanaRpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolanaRpcClient")
            .field("rpc_url", &self.rpc_url)
            .field("commitment", &self.commitment)
            .finish()
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
    fn test_network_display_names() {
        assert_eq!(SolanaNetwork::Mainnet.display_name(), "Mainnet Beta");
        assert_eq!(SolanaNetwork::Devnet.display_name(), "Devnet");
        assert_eq!(SolanaNetwork::Testnet.display_name(), "Testnet");
    }

    #[test]
    fn test_commitment_levels() {
        assert_eq!(RpcCommitment::Processed.as_str(), "processed");
        assert_eq!(RpcCommitment::Confirmed.as_str(), "confirmed");
        assert_eq!(RpcCommitment::Finalized.as_str(), "finalized");
    }

    #[test]
    fn test_transaction_signature() {
        let sig = TransactionSignature::new("test_signature".to_string());
        assert_eq!(sig.as_str(), "test_signature");

        let sig_from_string: TransactionSignature = "another_signature".to_string().into();
        assert_eq!(sig_from_string.as_str(), "another_signature");
    }

    #[test]
    fn test_rpc_client_creation() {
        let client = SolanaRpcClient::new(SolanaNetwork::Devnet);
        assert_eq!(client.rpc_url(), "https://api.devnet.solana.com");
        assert_eq!(client.commitment(), RpcCommitment::Confirmed);
    }

    #[test]
    fn test_rpc_client_custom_url() {
        let client = SolanaRpcClient::new_with_url(
            "https://custom-rpc.example.com",
            RpcCommitment::Finalized,
        );
        assert_eq!(client.rpc_url(), "https://custom-rpc.example.com");
        assert_eq!(client.commitment(), RpcCommitment::Finalized);
    }
}

// Re-export account service and mock types
pub mod account_service;
pub use account_service::{AccountService, AccountWithBalance, TransactionBuilder};

// Export mock Solana types (already defined above)
