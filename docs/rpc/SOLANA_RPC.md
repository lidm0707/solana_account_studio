# Solana RPC Documentation

## Overview

Solana RPC (Remote Procedure Call) nodes provide HTTP and WebSocket APIs for interacting with the Solana blockchain. They allow applications to query blockchain data, submit transactions, and receive real-time updates without participating in consensus.

## 🌊 MCP SurfPool Integration

This application now includes **MCP SurfPool integration** for local simnet development with mainnet forking capabilities. The Rust-only implementation provides:

- **Mainnet Fork**: Fork from live mainnet state for realistic testing
- **Local RPC**: HTTP RPC server on port 8999
- **WebSocket Support**: Real-time updates on port 9000
- **MCP Server**: Model Context Protocol integration for enhanced tooling
- **Pure Rust**: Complete Rust implementation without external dependencies

### Quick Start with SurfPool

```bash
# Start mainnet fork simnet
surfpool start --rpc-url https://api.mainnet-beta.solana.com --port 8999 --ws-port 9000 --no-tui

# Start MCP server (separate process)
surfpool mcp

# Test RPC connection
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getSlot","params":[]}' \
  http://localhost:8999
```

**Verified Test Results:**
- ✅ Mainnet fork operational (verified at slot 374098948)
- ✅ RPC endpoints responding correctly
- ✅ Balance queries functional: `{"result":{"value":20390245567}}`
- ✅ MCP server integration validated

## RPC Node Configuration

### Basic Validator Configuration

```bash
#!/bin/bash
exec solana-validator \
    --identity /home/sol/validator-keypair.json \
    --known-validator 5D1fNXzvv5NjV1ysLjirC4WY92RNsVH18vjmcszZd8on \
    --known-validator dDzy5SR3AXdYWVqbDEkVFdvSPCtS9ihF5kJkHCtXoFs \
    --known-validator eoKpUABi59aT4rR9HGS3LcMecfut9x7zJyodWWP43YQ \
    --known-validator Ft5fbkqNa76vnsjYNwjDZUXoTWpP7VYm3mtsaQckQADN \
    --known-validator 9QxCLckBiJc783jnMvXZubK4wH86Eqqvashtrwvcsgkv \
    --only-known-rpc \
    --full-rpc-api \
    --no-voting \
    --ledger /mnt/ledger \
    --accounts /mnt/accounts \
    --log /home/sol/solana-rpc.log \
    --rpc-port 8899 \
    --rpc-bind-address 0.0.0.0 \
    --private-rpc \
    --dynamic-port-range 8000-8020 \
    --entrypoint entrypoint.testnet.solana.com:8001 \
    --entrypoint entrypoint2.testnet.solana.com:8001 \
    --entrypoint entrypoint3.testnet.solana.com:8001 \
    --expected-genesis-hash 4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY \
    --wal-recovery-mode skip_any_corrupted_record \
    --limit-ledger-size
```

### Account Indexing for Performance

To improve RPC performance for account-scanning requests, enable account indexes:

```bash
--account-index program-id \
--account-index spl-token-mint \
--account-index spl-token-owner
```

**Supported Index Types:**
- `program-id`: Indexes accounts by owning program (used by `getProgramAccounts`)
- `spl-token-mint`: Indexes SPL token accounts by token Mint (used by `getTokenAccountsByDelegate`, `getTokenLargestAccounts`)
- `spl-token-owner`: Indexes SPL token accounts by token-owner address (used by `getTokenAccountsByOwner`, `getProgramAccounts` with spl-token-owner filter)

## RPC API Reference

### Core Account Methods

#### getAccountInfo
Retrieves information about a single account.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getAccountInfo",
  "params": [
    "YOUR_PUBLIC_KEY",
    {
      "encoding": "base64"
    }
  ]
}
```

#### getMultipleAccounts
Retrieves information for multiple accounts in a single request.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getMultipleAccounts",
  "params": [
    ["pubkey1", "pubkey2", "pubkey3"],
    {
      "encoding": "base64"
    }
  ]
}
```

#### getProgramAccounts
Retrieves all accounts owned by a specific program.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getProgramAccounts",
  "params": [
    "PROGRAM_ID",
    {
      "encoding": "base64",
      "filters": [
        {
          "memcmp": {
            "offset": 0,
            "bytes": "base64_encoded_bytes"
          }
        }
      ]
    }
  ]
}
```

### Transaction Methods

#### getTransaction
Retrieves transaction details by signature.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTransaction",
  "params": [
    "TRANSACTION_SIGNATURE",
    {
      "encoding": "json",
      "commitment": "confirmed"
    }
  ]
}
```

#### sendTransaction
Submits a signed transaction to the network.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sendTransaction",
  "params": [
    "BASE64_ENCODED_TRANSACTION",
    {
      "encoding": "base64",
      "preflightCommitment": "confirmed"
    }
  ]
}
```

### Token Methods

#### getTokenAccountBalance
Retrieves the balance of a token account.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenAccountBalance",
  "params": [
    "TOKEN_ACCOUNT_PUBKEY"
  ]
}
```

#### getTokenSupply
Retrieves the total supply of a token mint.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenSupply",
  "params": [
    "MINT_PUBKEY"
  ]
}
```

#### getTokenAccountsByOwner
Retrieves all token accounts owned by a specific address.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenAccountsByOwner",
  "params": [
    "OWNER_PUBKEY",
    {
      "mint": "MINT_PUBKEY"
    }
  ]
}
```

### Network Information Methods

#### getBalance
Retrieves the SOL balance of an account.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBalance",
  "params": [
    "ACCOUNT_PUBKEY",
    {
      "commitment": "confirmed"
    }
  ]
}
```

#### getRecentBlockhash
Retrieves the most recent blockhash for transaction creation.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getRecentBlockhash",
  "params": [
    {
      "commitment": "confirmed"
    }
  ]
}
```

#### getSlot
Retrieves the current slot.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getSlot",
  "params": [
    {
      "commitment": "confirmed"
    }
  ]
}
```

## WebSocket API

### Subscription Methods

#### accountSubscribe
Subscribe to account changes.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "accountSubscribe",
  "params": [
    "ACCOUNT_PUBKEY",
    {
      "encoding": "base64"
    }
  ]
}
```

#### signatureSubscribe
Subscribe to transaction confirmation updates.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "signatureSubscribe",
  "params": [
    "TRANSACTION_SIGNATURE",
    {
      "commitment": "confirmed"
    }
  ]
}
```

#### slotSubscribe
Subscribe to slot updates.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "slotSubscribe"
}
```

## Network Endpoints

### Mainnet Beta
- **HTTP**: `https://api.mainnet-beta.solana.com`
- **WebSocket**: `wss://api.mainnet-beta.solana.com`

### Devnet
- **HTTP**: `https://api.devnet.solana.com`
- **WebSocket**: `wss://api.devnet.solana.com`

### Testnet
- **HTTP**: `https://api.testnet.solana.com`
- **WebSocket**: `wss://api.testnet.solana.com`

## Configuration Examples

### Solana CLI Configuration

```bash
# Set to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Set to devnet
solana config set --url https://api.devnet.solana.com

# Set to testnet
solana config set --url https://api.testnet.solana.com

# Check current configuration
solana config get
```

### Environment Variables

```bash
# Default RPC URL
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"

# Custom RPC endpoint
export SOLANA_RPC_URL="http://localhost:8899"

# SurfPool Local Simnet (Recommended for Development)
export SOLANA_RPC_URL="http://localhost:8999"

# WebSocket URL
export SOLANA_WS_URL="wss://api.mainnet-beta.solana.com"

# SurfPool WebSocket (Local Development)
export SOLANA_WS_URL="ws://localhost:9000"
```

## Hardware Requirements for RPC Nodes

### Minimum Requirements
- **CPU**: 12 cores / 24 threads
- **RAM**: 128 GB
- **Storage**: 2TB NVMe SSD for ledger
- **Network**: 1 Gbps+ connection

### Recommended Requirements
- **CPU**: 16 cores / 32 threads or more
- **RAM**: 512 GB or more (if using account-index)
- **Storage**: 4TB+ NVMe SSD
- **Network**: 10 Gbps+ connection

## Performance Optimization

### Connection Pooling
- Use HTTP keep-alive connections
- Implement connection pooling for high-throughput applications
- Consider WebSocket connections for real-time updates

### Batch Requests
- Use `getMultipleAccounts` instead of multiple `getAccountInfo` calls
- Batch transaction submissions when possible
- Leverage account indexes for filtered queries

### Caching Strategy
- Cache frequently accessed account data
- Implement TTL-based cache invalidation
- Use program-specific caching for token accounts

## Error Handling

### Common Error Codes

#### -32602: Invalid params
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32602,
    "message": "Invalid params"
  },
  "id": 1
}
```

#### -32002: Node is behind by X slots
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32002,
    "message": "Node is behind by 42 slots"
  },
  "id": 1
}
```

#### -32007: Slot was skipped
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32007,
    "message": "Slot was skipped"
  },
  "id": 1
}
```

### Retry Strategy
- Implement exponential backoff for rate limiting
- Use commitment levels (`confirmed`, `finalized`) appropriately
- Handle network timeouts gracefully

## Security Considerations

### Private RPC Nodes
- Use `--private-rpc` flag for restricted access
- Implement authentication for private endpoints
- Monitor RPC endpoint usage and access patterns

### Rate Limiting
- Public RPC endpoints have rate limits
- Consider running private RPC nodes for production applications
- Implement request queuing for high-volume applications

## Integration Examples

### Rust Client Example (SurfPool Integration)

```rust
use surfdesk_core::solana_rpc::{SolanaRpcClient, Pubkey};

// Connect to SurfPool mainnet fork
let rpc_url = "http://localhost:8999";
let client = SolanaRpcClient::new_with_url(rpc_url, RpcCommitment::Confirmed);

// Get account info
let pubkey = Pubkey::from_string("YOUR_PUBLIC_KEY")?;
let account = client.get_account_info(&pubkey.to_string()).await?;

// Get balance
let balance = client.get_balance(&pubkey.to_string()).await?;

// Get latest blockhash
let blockhash = client.get_latest_blockhash().await?;

// Deploy via SurfPool (MCP integration)
let program_id = client.deploy_via_surfpool(&program_bytes).await?;
```

### Traditional Solana Client Example

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

let rpc_url = "https://api.mainnet-beta.solana.com";
let client = RpcClient::new(rpc_url);

// Get account info
let pubkey = Pubkey::from_str("YOUR_PUBLIC_KEY")?;
let account = client.get_account(&pubkey)?;

// Get balance
let balance = client.get_balance(&pubkey)?;

// Get recent blockhash
let blockhash = client.get_latest_blockhash()?;
```

### JavaScript Client Example

```javascript
import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com');

// Get account info
const publicKey = new PublicKey('YOUR_PUBLIC_KEY');
const accountInfo = await connection.getAccountInfo(publicKey);

// Get balance
const balance = await connection.getBalance(publicKey);

// Get recent blockhash
const blockhash = await connection.getLatestBlockhash();
```

## Best Practices

1. **Use appropriate commitment levels**: `confirmed` for most use cases, `finalized` for critical operations
2. **Implement proper error handling**: Handle network errors, timeouts, and rate limits
3. **Optimize request patterns**: Batch requests, use WebSockets for real-time updates
4. **Monitor performance**: Track RPC call latency and success rates
5. **Use account indexes**: Enable relevant indexes for better query performance
6. **Consider private RPC nodes**: For production applications requiring high throughput
7. **Leverage SurfPool for development**: Use local mainnet forks for realistic testing without cost
8. **Utilize MCP integration**: Take advantage of Model Context Protocol for enhanced tooling
9. **Pure Rust architecture**: Prefer the built-in rust-only implementation for better performance and security

## 🚀 Development Workflow with SurfPool

### Recommended Development Setup

1. **Start SurfPool with mainnet fork**:
   ```bash
   surfpool start --rpc-url https://api.mainnet-beta.solana.com --port 8999 --no-tui
   ```

2. **Configure application for local development**:
   ```bash
   export SOLANA_RPC_URL="http://localhost:8999"
   export SOLANA_WS_URL="ws://localhost:9000"
   ```

3. **Use Rust-only solana_rpc module**:
   ```rust
   let client = SolanaRpcClient::new_with_url("http://localhost:8999", RpcCommitment::Confirmed);
   ```

4. **Deploy and test programs locally**:
   ```rust
   let program_id = client.deploy_via_surfpool(&program_bytes).await?;
   ```

### Benefits of Rust-Only Implementation

- **Performance**: Native Rust performance without FFI overhead
- **Security**: Memory safety and type safety guarantees
- **Integration**: Seamless integration with existing Rust codebase
- **Maintenance**: Single language stack reduces complexity
- **MCP Support**: Built-in Model Context Protocol integration