# Solana RPC Integration Guide

## Table of Contents
1. [Quick Start](#quick-start)
2. [Client Setup](#client-setup)
3. [Authentication](#authentication)
4. [Common Operations](#common-operations)
5. [Error Handling](#error-handling)
6. [Performance Optimization](#performance-optimization)
7. [WebSocket Integration](#websocket-integration)
8. [Testing](#testing)
9. [Migration Guide](#migration-guide)

## Quick Start

### Basic Connection

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

// Create RPC client with default commitment
let client = RpcClient::new("https://api.mainnet-beta.solana.com");

// Create RPC client with specific commitment
let client = RpcClient::new_with_commitment(
    "https://api.mainnet-beta.solana.com",
    CommitmentConfig::confirmed()
);
```

```javascript
import { Connection, Commitment } from '@solana/web3.js';

// Create connection with default commitment
const connection = new Connection('https://api.mainnet-beta.solana.com');

// Create connection with specific commitment
const connection = new Connection(
    'https://api.mainnet-beta.solana.com',
    'confirmed'
);
```

## Client Setup

### Environment Configuration

```bash
# .env file
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_WS_URL=wss://api.mainnet-beta.solana.com
SOLANA_COMMITMENT=confirmed
SOLANA_TIMEOUT=30000

# MCP SurfPool Local Development (Recommended)
SOLANA_RPC_URL=http://localhost:8999
SOLANA_WS_URL=ws://localhost:9000
```

### Advanced Configuration

#### Rust-Only Implementation (Recommended)

```rust
use surfdesk_core::solana_rpc::{SolanaRpcClient, RpcCommitment};

// Connect to MCP SurfPool for local development
let client = SolanaRpcClient::new_with_url("http://localhost:8999", RpcCommitment::Confirmed);

// Or connect to mainnet for production
let mainnet_client = SolanaRpcClient::new_with_url("https://api.mainnet-beta.solana.com", RpcCommitment::Confirmed);
```

#### Traditional Solana Client

```rust
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use std::time::Duration;

// Configure custom timeout
let client = RpcClient::new_with_timeout(
    "https://api.mainnet-beta.solana.com",
    Duration::from_secs(30)
);

// Configure with custom user agent
let client = RpcClient::new_with_user_agent(
    "https://api.mainnet-beta.solana.com",
    "SurfDesk/1.0.0"
);
```

```javascript
import { Connection } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com', {
    commitment: 'confirmed',
    httpHeaders: {
        'User-Agent': 'SurfDesk/1.0.0'
    },
    fetchMiddleware: (fetch, url, options) => {
        // Custom middleware for logging, retries, etc.
        console.log(`RPC Request: ${url}`);
        return fetch(url, options);
    }
});
```

## Authentication

### API Key Authentication

```rust
use solana_client::rpc_client::RpcClient;

// Custom headers for API key
let client = RpcClient::new_with_commitment(
    "https://your-rpc-provider.com/v1/YOUR_API_KEY",
    CommitmentConfig::confirmed()
);
```

```javascript
import { Connection } from '@solana/web3.js';

const connection = new Connection('https://your-rpc-provider.com/v1/YOUR_API_KEY', {
    httpHeaders: {
        'Authorization': 'Bearer YOUR_API_KEY'
    }
});
```

### WebSocket Authentication

```javascript
import { Connection } from '@solana/web3.js';

const connection = new Connection('wss://your-rpc-provider.com/ws/YOUR_API_KEY', {
    wsEndpoint: 'wss://your-rpc-provider.com/ws/YOUR_API_KEY',
    httpHeaders: {
        'Authorization': 'Bearer YOUR_API_KEY'
    }
});
```

## Common Operations

### Account Operations

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

let client = RpcClient::new("https://api.mainnet-beta.solana.com");
let pubkey = Pubkey::from_str("YOUR_PUBLIC_KEY")?;

// Get account info
let account = client.get_account(&pubkey)?;

// Get account balance
let balance = client.get_balance(&pubkey)?;

// Get multiple accounts
let pubkeys = vec![
    Pubkey::from_str("PUBKEY_1")?,
    Pubkey::from_str("PUBKEY_2")?,
    Pubkey::from_str("PUBKEY_3")?,
];
let accounts = client.get_multiple_accounts(&pubkeys)?;
```

```javascript
import { Connection, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com');
const publicKey = new PublicKey('YOUR_PUBLIC_KEY');

// Get account info
const accountInfo = await connection.getAccountInfo(publicKey);

// Get account balance
const balance = await connection.getBalance(publicKey);
const balanceInSol = balance / LAMPORTS_PER_SOL;

// Get multiple accounts
const publicKeys = [
    new PublicKey('PUBKEY_1'),
    new PublicKey('PUBKEY_2'),
    new PublicKey('PUBKEY_3')
];
const accounts = await connection.getMultipleAccountsInfo(publicKeys);
```

### Transaction Operations

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    transaction::Transaction,
    signature::Signer,
    commitment_config::CommitmentConfig
};

let client = RpcClient::new_with_commitment(
    "https://api.mainnet-beta.solana.com",
    CommitmentConfig::confirmed()
);

// Send transaction
let signature = client.send_and_confirm_transaction(&transaction)?;

// Get transaction info
let transaction_info = client.get_transaction(
    &signature,
    UiTransactionEncoding::Json
)?;

// Get recent blockhash
let blockhash = client.get_latest_blockhash()?;
```

```javascript
import { Connection, sendAndConfirmTransaction } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

// Send and confirm transaction
const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [signer]
);

// Get transaction info
const transactionInfo = await connection.getTransaction(signature, {
    commitment: 'confirmed',
    maxSupportedTransactionVersion: 0
});

// Get recent blockhash
const blockhash = await connection.getLatestBlockhash();
```

### Program Operations

```rust
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcProgramAccountsConfig, RpcAccountInfoConfig};
use solana_sdk::pubkey::Pubkey;

let client = RpcClient::new("https://api.mainnet-beta.solana.com");
let program_id = Pubkey::from_str("PROGRAM_ID")?;

// Get all program accounts
let config = RpcProgramAccountsConfig {
    account_config: RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        ..RpcAccountInfoConfig::default()
    },
    filters: None,
    with_context: None,
};

let accounts = client.get_program_accounts(&program_id, config)?;
```

```javascript
import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com');
const programId = new PublicKey('PROGRAM_ID');

// Get all program accounts
const accounts = await connection.getProgramAccounts(programId, {
    encoding: 'base64',
    filters: [
        {
            memcmp: {
                offset: 0,
                bytes: 'base64_encoded_filter'
            }
        }
    ]
});
```

## Error Handling

### Rust Error Handling

```rust
use solana_client::client_error::ClientError;
use solana_client::rpc_request::RpcError;
use std::time::Duration;
use tokio::time::sleep;

async fn retry_rpc_call<F, T>(mut f: F) -> Result<T, ClientError>
where
    F: FnMut() -> Result<T, ClientError>,
{
    let mut retries = 3;
    let mut delay = Duration::from_millis(1000);

    loop {
        match f() {
            Ok(result) => return Ok(result),
            Err(ClientError::Reqwest(e)) if e.is_timeout() => {
                if retries == 0 {
                    return Err(ClientError::Reqwest(e));
                }
                retries -= 1;
                sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
            Err(ClientError::RpcError(RpcError::RpcResponseError { code, .. })) 
                if code == -32002 => { // Node is behind
                if retries == 0 {
                    return Err(ClientError::RpcError(RpcError::RpcResponseError {
                        code,
                        message: "Node is behind and retries exhausted".to_string(),
                        data: None,
                    }));
                }
                retries -= 1;
                sleep(delay).await;
                delay *= 2;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### JavaScript Error Handling

```javascript
import { Connection } from '@solana/web3.js';

class RpcClient {
    constructor(url, options = {}) {
        this.connection = new Connection(url, options);
        this.maxRetries = options.maxRetries || 3;
        this.retryDelay = options.retryDelay || 1000;
    }

    async callWithRetry(method, ...args) {
        let retries = this.maxRetries;
        let delay = this.retryDelay;

        while (retries > 0) {
            try {
                return await this.connection[method](...args);
            } catch (error) {
                if (error.name === 'ConnectionTimeoutError' || 
                    error.code === -32002) { // Node is behind
                    if (retries === 0) throw error;
                    
                    await new Promise(resolve => setTimeout(resolve, delay));
                    retries--;
                    delay *= 2; // Exponential backoff
                } else {
                    throw error;
                }
            }
        }
    }

    async getBalance(publicKey) {
        return this.callWithRetry('getBalance', publicKey);
    }

    async getAccountInfo(publicKey) {
        return this.callWithRetry('getAccountInfo', publicKey);
    }
}
```

## Performance Optimization

### Connection Pooling

```rust
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use tokio::sync::RwLock;

struct RpcPool {
    clients: Vec<Arc<RpcClient>>,
    current_index: std::sync::atomic::AtomicUsize,
}

impl RpcPool {
    fn new(urls: Vec<String>) -> Self {
        let clients = urls
            .into_iter()
            .map(|url| Arc::new(RpcClient::new(url)))
            .collect();

        Self {
            clients,
            current_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn get_client(&self) -> Arc<RpcClient> {
        let index = self.current_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            % self.clients.len();
        self.clients[index].clone()
    }
}
```

```javascript
class RpcConnectionPool {
    constructor(urls, options = {}) {
        this.connections = urls.map(url => new Connection(url, options));
        this.currentIndex = 0;
    }

    getConnection() {
        const connection = this.connections[this.currentIndex];
        this.currentIndex = (this.currentIndex + 1) % this.connections.length;
        return connection;
    }

    async batchCall(method, args) {
        const connection = this.getConnection();
        return connection[method](...args);
    }
}
```

### Batch Requests

```rust
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use solana_client::rpc_response::RpcResponse;
use serde_json::{json, Value};

async fn batch_get_accounts(client: &RpcClient, pubkeys: Vec<String>) -> Result<Vec<Value>, ClientError> {
    let requests: Vec<Value> = pubkeys
        .into_iter()
        .enumerate()
        .map(|(i, pubkey)| json!({
            "jsonrpc": "2.0",
            "id": i,
            "method": "getAccountInfo",
            "params": [pubkey, {"encoding": "base64"}]
        }))
        .collect();

    let batch_request = json!(requests);
    let response: Vec<RpcResponse<Value>> = client.send::<Vec<Value>>(
        RpcRequest::Custom(String::new()),
        serde_json::to_value(batch_request)?
    )?;

    Ok(response.into_iter().map(|r| r.result).collect())
}
```

```javascript
import { Connection, PublicKey } from '@solana/web3.js';

class BatchRpcClient {
    constructor(connection) {
        this.connection = connection;
    }

    async getMultipleAccountsBatch(publicKeys, batchSize = 100) {
        const results = [];
        
        for (let i = 0; i < publicKeys.length; i += batchSize) {
            const batch = publicKeys.slice(i, i + batchSize);
            const batchResults = await this.connection.getMultipleAccountsInfo(batch);
            results.push(...batchResults);
        }
        
        return results;
    }

    async getBalancesBatch(publicKeys, batchSize = 100) {
        const results = [];
        
        for (let i = 0; i < publicKeys.length; i += batchSize) {
            const batch = publicKeys.slice(i, i + batchSize);
            const promises = batch.map(pubkey => this.connection.getBalance(pubkey));
            const batchResults = await Promise.all(promises);
            results.push(...batchResults);
        }
        
        return results;
    }
}
```

## WebSocket Integration

### Real-time Account Updates

```rust
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_subscription::RpcSubscription;
use solana_sdk::pubkey::Pubkey;
use tokio_stream::StreamExt;

let client = RpcClient::new("wss://api.mainnet-beta.solana.com");
let pubkey = Pubkey::from_str("YOUR_PUBLIC_KEY")?;

// Subscribe to account changes
let (mut subscription, _unsubscribe) = client.account_subscribe(&pubkey, UiAccountEncoding::Base64)?;

while let Some(account_update) = subscription.next().await {
    match account_update {
        Ok(account) => {
            println!("Account updated: {:?}", account);
            // Handle account update
        }
        Err(e) => {
            eprintln!("Error in subscription: {:?}", e);
            break;
        }
    }
}
```

```javascript
import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection('wss://api.mainnet-beta.solana.com');
const publicKey = new PublicKey('YOUR_PUBLIC_KEY');

// Subscribe to account changes
const subscriptionId = await connection.onAccountChange(
    publicKey,
    (accountInfo) => {
        console.log('Account updated:', accountInfo);
        // Handle account update
    },
    'confirmed'
);

// Unsubscribe when done
await connection.removeAccountChangeListener(subscriptionId);
```

### Transaction Monitoring

```javascript
import { Connection } from '@solana/web3.js';

const connection = new Connection('wss://api.mainnet-beta.solana.com');

// Monitor transaction confirmations
const subscriptionId = await connection.onSignature(
    'TRANSACTION_SIGNATURE',
    (signature, result) => {
        console.log('Transaction confirmed:', signature, result);
        // Handle transaction confirmation
    },
    'confirmed'
);

// Unsubscribe
await connection.removeSignatureListener(subscriptionId);
```

## Testing

### MCP SurfPool Testing

```rust
#[cfg(test)]
mod tests {
    use surfdesk_core::solana_rpc::{SolanaRpcClient, Pubkey, RpcCommitment};
    
    #[tokio::test]
    async fn test_surfpool_integration() {
        // Test with local SurfPool instance
        let client = SolanaRpcClient::new_with_url("http://localhost:8999", RpcCommitment::Confirmed);
        
        // Test connection
        let is_connected = client.test_connection().await.unwrap();
        assert!(is_connected);
        
        // Test slot retrieval (verifies mainnet fork)
        let blockhash = client.get_latest_blockhash().await.unwrap();
        println!("Connected to mainnet fork at blockhash: {}", blockhash);
        
        // Test account balance
        let system_program = Pubkey::from_string("11111111111111111111111111111112").unwrap();
        let balance = client.get_balance(&system_program.to_string()).await.unwrap();
        assert!(balance > 0);
        println!("System program balance: {} lamports", balance);
    }
    
    #[tokio::test]
    async fn test_program_deployment() {
        let client = SolanaRpcClient::new_with_url("http://localhost:8999", RpcCommitment::Confirmed);
        
        // Test program deployment via SurfPool
        let mock_program = vec![0x01, 0x02, 0x03, 0x04]; // Mock program bytes
        let program_id = client.deploy_via_surfpool(&mock_program).await.unwrap();
        
        println!("Deployed program with ID: {}", program_id);
        
        // Verify program exists
        let account_info = client.get_account_info(&program_id.to_string()).await.unwrap();
        assert!(account_info.is_some());
        assert!(account_info.unwrap().executable);
    }
}
```

### Mock RPC for Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    #[tokio::test]
    async fn test_account_operations() {
        let client = RpcClient::new("https://api.devnet.solana.com");
        let pubkey = Pubkey::from_str("11111111111111111111111111111112").unwrap();
        
        // Test account info
        let account = client.get_account(&pubkey).unwrap();
        assert!(account.is_some());
        
        // Test balance
        let balance = client.get_balance(&pubkey).unwrap();
        assert!(balance >= 0);
    }
}
```

```javascript
describe('RPC Integration Tests', () => {
  let connection;
  
  beforeEach(() => {
    connection = new Connection('https://api.devnet.solana.com');
  });
  
  test('should get account info', async () => {
    const publicKey = new PublicKey('11111111111111111111111111111112');
    const accountInfo = await connection.getAccountInfo(publicKey);
    expect(accountInfo).toBeDefined();
  });
  
  test('should get balance', async () => {
    const publicKey = new PublicKey('11111111111111111111111111111112');
    const balance = await connection.getBalance(publicKey);
    expect(balance).toBeGreaterThanOrEqual(0);
  });
});
```

## Migration Guide

### From Legacy RPC to Current Version

#### Breaking Changes
- `getConfirmedBlock` → `getBlock`
- `getConfirmedTransaction` → `getTransaction`
- `getConfirmedSignaturesForAddress` → `getSignaturesForAddress`

#### Migration Example

```rust
// Old way (deprecated)
let block = client.get_confirmed_block(slot)?;
let transaction = client.get_confirmed_transaction(&signature)?;

// New way (current)
let block = client.get_block_with_config(
    slot,
    GetBlockConfig {
        encoding: None,
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(true),
        commitment: Some(CommitmentConfig::confirmed()),
    }
)?;
let transaction = client.get_transaction_with_config(
    &signature,
    UiTransactionEncoding::Json,
    RpcTransactionConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    }
)?;
```

```javascript
// Old way (deprecated)
const block = await connection.getConfirmedBlock(slot);
const transaction = await connection.getConfirmedTransaction(signature);

// New way (current)
const block = await connection.getBlock(slot, {
    commitment: 'confirmed',
    maxSupportedTransactionVersion: 0
});
const transaction = await connection.getTransaction(signature, {
    commitment: 'confirmed',
    maxSupportedTransactionVersion: 0
});
```

### Configuration Migration

```toml
# Old configuration
[network]
url = "https://api.mainnet-beta.solana.com"
commitment = "single"

# New configuration
[network]
default_rpc = "https://api.mainnet-beta.solana.com"
network = "mainnet-beta"
commitment = "confirmed"  # processed, confirmed, finalized
websocket_url = "wss://api.mainnet-beta.solana.com"
```

## Best Practices Summary

1. **Always specify commitment levels** appropriate for your use case
2. **Implement proper error handling** with exponential backoff
3. **Use batch requests** for multiple account queries
4. **Enable WebSocket subscriptions** for real-time updates
5. **Test against devnet** before mainnet deployment
6. **Monitor RPC usage** and implement rate limiting
7. **Use connection pooling** for high-throughput applications
8. **Cache frequently accessed data** to reduce RPC calls
9. **Handle network failures** gracefully with retry logic
10. **Keep dependencies updated** to access latest RPC features
```
