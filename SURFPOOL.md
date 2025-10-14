# Surfpool: Local Solana Development Controller

## Overview

Surfpool is the core local development controller component of SurfDesk that manages and orchestrates Solana development environments. It provides a unified interface for running local test validators, managing network configurations, and controlling the development lifecycle for Solana programs.

## Core Capabilities

### 🏊 **Local Test Validator Management**

Surfpool manages Solana's `solana-test-validator` process, providing developers with a complete local blockchain environment that mirrors mainnet behavior without requiring external connections.

#### **Key Features:**
- **One-Click Validator Startup**: Instantly launch a local Solana validator with pre-configured settings
- **Multi-Environment Support**: Run multiple validator instances simultaneously (devnet, fork, custom)
- **Automatic Port Management**: Handles port allocation and conflict resolution
- **Resource Monitoring**: Tracks CPU, memory, and disk usage of validator processes
- **Health Checks**: Continuous monitoring of validator status and connectivity

#### **Validator Configurations:**
```bash
# Standard Devnet
solana-test-validator --reset --quiet

# Mainnet Fork
solfana-test-validator --url https://api.mainnet-beta.solana.com --rpc-port 8899

# Custom Genesis
solana-test-validator --clone-upgradeable-program <PROGRAM_ID> --bpf-program <PATH>

# Time-Controlled
solana-test-validator --warp-slot <SLOT_NUMBER>
```

### 🌐 **Network Environment Control**

Surfpool provides sophisticated network management capabilities that enable developers to create and switch between different development environments seamlessly.

#### **Environment Types:**

**1. Local Devnet**
- Fresh validator instance with default genesis
- Pre-funded accounts for testing
- Built-in program library (System, Token, etc.)
- Ideal for new program development

**2. Mainnet Fork**
- Real-time fork of mainnet state
- Access to live programs and accounts
- Historical data preservation
- Perfect for testing against production conditions

**3. Custom Networks**
- User-defined genesis configurations
- Custom program deployments
- Specific validator settings
- Tailored for specialized testing scenarios

**4. Time-Controlled Environments**
- Slot manipulation capabilities
- State snapshot management
- Historical state exploration
- Essential for debugging time-sensitive issues

### ⚡ **Program Deployment & Management**

Surfpool streamlines the program development lifecycle with automated deployment and management features.

#### **Deployment Features:**
- **Build Integration**: Automatically builds Rust programs using Cargo
- **Account Management**: Creates and manages program accounts
- **Upgrade Support**: Handles program upgrades with proper authority
- **ID Generation**: Generates deterministic program IDs
- **Deployment History**: Tracks all deployment versions and changes

#### **Program Operations:**
```rust
// Deploy new program
let program_id = surfpool.deploy_program("target/deploy/my_program.so")?;

// Upgrade existing program
surfpool.upgrade_program(program_id, "target/deploy/my_program_v2.so")?;

// Close program
surfpool.close_program(program_id)?;
```

### 🔄 **State Management & Snapshots**

Surfpool provides comprehensive state management capabilities that enable developers to capture, restore, and manipulate blockchain state.

#### **State Features:**
- **Account Snapshots**: Capture complete account state at any point
- **Program State**: Save and restore program-specific data
- **Slot Snapshots**: Create state snapshots at specific slots
- **Selective Restore**: Restore specific accounts or programs
- **State Diffing**: Compare state changes between snapshots

#### **Snapshot Operations:**
```bash
# Create account snapshot
surfpool snapshot create --accounts <ACCOUNT_LIST> --name "pre_upgrade"

# Restore from snapshot
surfpool snapshot restore --name "pre_upgrade"

# List available snapshots
surfpool snapshot list
```

### ⏰ **Time Control & Slot Manipulation**

Surfpool's time control features give developers unprecedented ability to manipulate blockchain time for testing and debugging.

#### **Time Features:**
- **Slot Warping**: Jump to specific slots for testing
- **Time Acceleration**: Speed up block production
- **Slot Rewinding**: Go back to previous slots
- **Block Height Control**: Manipulate block production
- **Timestamp Manipulation**: Control block timestamps

#### **Time Operations:**
```bash
# Warp to specific slot
surfpool time warp-slot 123456789

# Advance slots
surfpool time advance-slots 100

# Rewind slots
surfpool time rewind-slots 50

# Set slot speed
surfpool time set-speed 2x
```

### 💾 **Account & Data Management**

Surfpool provides comprehensive account management tools that make it easy to work with Solana accounts and data.

#### **Account Features:**
- **Account Creation**: Create various account types (system, token, custom)
- **Balance Management**: Fund accounts with SOL for testing
- **Data Manipulation**: Directly modify account data
- **Account Cloning**: Copy account data between environments
- **Rent Management**: Handle rent payments and exemptions

#### **Account Operations:**
```rust
// Create system account
let account = surfpool.create_account(SystemProgram, &account_key)?;

// Fund account
surfpool.fund_account(&account, 10_000_000_000)?; // 10 SOL

// Modify account data
surfpool.modify_account_data(&account, new_data)?;
```

### 🔌 **RPC & API Integration**

Surfpool provides local RPC endpoints that are fully compatible with Solana's JSON-RPC API, enabling seamless integration with existing tools and frameworks.

#### **RPC Features:**
- **Full RPC Compatibility**: Supports all standard Solana RPC methods
- **Custom Endpoints**: Additional debugging and management endpoints
- **WebSocket Support**: Real-time account and transaction updates
- **Rate Limiting**: Configurable rate limits for testing
- **Request Logging**: Complete request/response logging

#### **Available RPC Methods:**
- `getAccountInfo`
- `getProgramAccounts`
- `sendTransaction`
- `simulateTransaction`
- `getLatestBlockhash`
- `getSlot`
- And all standard Solana RPC methods

### 🛠️ **Development Tools Integration**

Surfpool integrates seamlessly with popular Solana development tools and frameworks.

#### **Tool Integrations:**
- **Anchor Framework**: Automatic Anchor workspace detection and management
- **Solana CLI**: Direct integration with `solana-cli` commands
- **TypeScript SDK**: Native support for `@solana/web3.js`
- **Rust SDK**: Integration with `solana-sdk-rust`
- **IDE Plugins**: VS Code and other IDE plugin support

#### **IDE Features:**
- **Syntax Highlighting**: Solana-specific syntax highlighting
- **Code Completion**: Auto-completion for Solana APIs
- **Error Detection**: Real-time error checking and suggestions
- **Debug Integration**: Direct debugging from IDE
- **Testing Support**: Built-in test runner integration

## Architecture Overview

### **Core Components**

#### **1. Validator Controller**
```rust
pub struct ValidatorController {
    validator_process: Child,
    config: ValidatorConfig,
    health_monitor: HealthMonitor,
    rpc_server: RpcServer,
}
```

#### **2. Environment Manager**
```rust
pub struct EnvironmentManager {
    environments: HashMap<String, Environment>,
    active_environment: Option<String>,
    state_manager: StateManager,
}
```

#### **3. Account Manager**
```rust
pub struct AccountManager {
    accounts: HashMap<Pubkey, Account>,
    rent_collector: RentCollector,
    balance_manager: BalanceManager,
}
```

#### **4. Time Controller**
```rust
pub struct TimeController {
    current_slot: u64,
    slot_history: Vec<SlotInfo>,
    time_speed: f64,
    warp_history: Vec<WarpOperation>,
}
```

### **Data Flow**

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   SurfDesk UI   │◄──►│   Surfpool Core   │◄──►│  Test Validator │
└─────────────────┘    └──────────────────┘    └─────────────────┘
          │                      │                      │
          ▼                      ▼                      ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Configuration │    │  State Management │    │  RPC Server    │
│   Management   │    │     System        │    │                │
└─────────────────┘    └──────────────────┘    └─────────────────┘
          │                      │                      │
          ▼                      ▼                      ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Database      │    │   File System     │    │  Network Stack  │
│   (SQLite)      │    │   (Snapshots)     │    │  (TCP/UDP)      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Configuration

### **Validator Configuration**

```toml
[validator]
# Basic settings
rpc_port = 8899
ws_port = 8900
gossip_port = 8001
tpu_port = 8002

# Performance settings
threads = 4
ram = 4096
ledger_path = "./ledger"

# Genesis settings
genesis_hash = "default"
account_paths = ["./accounts"]
bpf_programs = ["./programs"]

# Fork settings (optional)
fork_url = "https://api.mainnet-beta.solana.com"
fork_slot = 123456789

# Time control settings
enable_time_control = true
default_time_speed = 1.0
auto_warp_interval = 100
```

### **Environment Configuration**

```toml
[environments.local]
type = "devnet"
validator_port = 8899
genesis = "default"
accounts = [
    { key = "11111111111111111111111111111111", balance = 1000000000 },
    { key = "22222222222222222222222222222222", balance = 500000000 }
]

[environments.mainnet_fork]
type = "fork"
url = "https://api.mainnet-beta.solana.com"
slot = 123456789
validator_port = 8898
accounts = "forked"

[environments.custom]
type = "custom"
genesis_file = "./custom_genesis.json"
validator_port = 8897
accounts = "./custom_accounts.json"
```

## Usage Examples

### **Basic Development Workflow**

```rust
use surfpool::{Surfpool, EnvironmentType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Surfpool
    let mut surfpool = Surfpool::new().await?;
    
    // Create local devnet environment
    let env_id = surfpool.create_environment(EnvironmentType::LocalDevnet).await?;
    
    // Deploy a program
    let program_id = surfpool.deploy_program("./target/deploy/my_program.so").await?;
    
    // Create test accounts
    let test_accounts = surfpool.create_test_accounts(10).await?;
    
    // Run tests
    surfpool.run_tests("./tests/").await?;
    
    // Create snapshot
    let snapshot_id = surfpool.create_snapshot("pre-upgrade").await?;
    
    // Upgrade program
    surfpool.upgrade_program(program_id, "./target/deploy/my_program_v2.so").await?;
    
    // Run tests again
    surfpool.run_tests("./tests/").await?;
    
    // Restore snapshot if needed
    if !surfpool.tests_passed() {
        surfpool.restore_snapshot(snapshot_id).await?;
    }
    
    Ok(())
}
```

### **Advanced Time Control**

```rust
use surfpool::time::{TimeController, WarpOperation};

async fn test_time_sensitive_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let mut surfpool = Surfpool::new().await?;
    let time_controller = surfpool.get_time_controller();
    
    // Warp to specific slot
    time_controller.warp_to_slot(123456789).await?;
    
    // Test program behavior at this slot
    let results = surfpool.run_time_sensitive_tests().await?;
    
    // Rewind and test again
    time_controller.rewind_slots(100).await?;
    let results2 = surfpool.run_time_sensitive_tests().await?;
    
    // Compare results
    let comparison = compare_test_results(results, results2)?;
    
    Ok(())
}
```

### **Fork Testing**

```rust
async fn test_against_mainnet() -> Result<(), Box<dyn std::error::Error>> {
    let mut surfpool = Surfpool::new().await?;
    
    // Create mainnet fork
    let env_id = surfpool.create_environment(EnvironmentType::MainnetFork {
        url: "https://api.mainnet-beta.solana.com",
        slot: 123456789,
    }).await?;
    
    // Get live account data
    let account = surfpool.get_account("SerumV2Pool1111111111111111111111111111111").await?;
    
    // Test against live data
    let results = surfpool.test_with_live_data(account).await?;
    
    // Create snapshot for repeated testing
    let snapshot_id = surfpool.create_snapshot("mainnet_state").await?;
    
    // Run tests multiple times with same state
    for i in 0..10 {
        surfpool.restore_snapshot(snapshot_id).await?;
        let test_results = surfpool.run_tests("./tests/").await?;
        println!("Test run {} results: {:?}", i, test_results);
    }
    
    Ok(())
}
```

## Integration with SurfDesk

### **Tauri Integration**

Surfpool integrates with SurfDesk's Tauri frontend through a comprehensive Rust API:

```rust
#[tauri::command]
async fn create_environment(environment_type: String) -> Result<String, String> {
    let surfpool = get_surfpool_instance().await?;
    let env_type = parse_environment_type(&environment_type)?;
    let env_id = surfpool.create_environment(env_type).await
        .map_err(|e| e.to_string())?;
    Ok(env_id)
}

#[tauri::command]
async fn deploy_program(program_path: String) -> Result<String, String> {
    let surfpool = get_surfpool_instance().await?;
    let program_id = surfpool.deploy_program(&program_path).await
        .map_err(|e| e.to_string())?;
    Ok(program_id.to_string())
}

#[tauri::command]
async fn get_validator_status() -> Result<ValidatorStatus, String> {
    let surfpool = get_surfpool_instance().await?;
    let status = surfpool.get_validator_status().await
        .map_err(|e| e.to_string())?;
    Ok(status)
}
```

### **Database Integration**

Surfpool stores all environment and state data in SurfDesk's SQLite database:

```sql
-- Environments table
CREATE TABLE environments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    config JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Deployments table
CREATE TABLE deployments (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    program_id TEXT NOT NULL,
    program_path TEXT NOT NULL,
    deployment_slot INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (environment_id) REFERENCES environments(id)
);

-- Snapshots table
CREATE TABLE snapshots (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    name TEXT NOT NULL,
    slot INTEGER NOT NULL,
    accounts JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (environment_id) REFERENCES environments(id)
);
```

## Best Practices

### **Development Workflow**

1. **Use Separate Environments**: Create dedicated environments for different testing scenarios
2. **Regular Snapshots**: Create snapshots before major changes to enable quick rollbacks
3. **Clean State Management**: Start with a clean slate for each testing session
4. **Resource Monitoring**: Keep an eye on validator resource usage to prevent issues
5. **Regular Updates**: Keep Surfpool and Solana tools up to date

### **Performance Optimization**

1. **Adjust Validator Resources**: Configure validator resources based on project needs
2. **Use SSD Storage**: Store ledger data on fast SSD storage for better performance
3. **Limit Account History**: Configure appropriate account history limits
4. **Optimize RPC Settings**: Tune RPC server settings for your use case
5. **Monitor Network Traffic**: Keep an eye on network usage and optimize accordingly

### **Security Considerations**

1. **Secure Private Keys**: Never commit private keys to version control
2. **Use Test Keys**: Generate dedicated test keys for development
3. **Isolate Development**: Keep development environments isolated from production
4. **Regular Cleanup**: Regularly clean up old environments and snapshots
5. **Access Control**: Implement proper access controls for team environments

## Troubleshooting

### **Common Issues**

#### **Validator Won't Start**
```bash
# Check port availability
netstat -tulpn | grep :8899

# Check system resources
free -h
df -h

# Check logs
surfpool logs --level debug
```

#### **RPC Connection Issues**
```bash
# Test RPC endpoint
curl -X POST http://localhost:8899 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Check WebSocket connection
wscat -c ws://localhost:8900
```

#### **Account Data Issues**
```bash
# Check account data
surfpool account get <ACCOUNT_ADDRESS>

# Verify account ownership
surfpool account verify <ACCOUNT_ADDRESS>

# Check account rent
surfpool account rent <ACCOUNT_ADDRESS>
```

### **Debug Mode**

Enable debug mode for detailed logging:

```rust
let mut surfpool = Surfpool::new()
    .with_debug_mode(true)
    .with_log_level(LogLevel::Debug)
    .await?;
```

## Future Enhancements

### **Planned Features**

1. **Multi-Validator Support**: Run multiple validators in a cluster
2. **Cross-Chain Testing**: Support for testing cross-chain interactions
3. **Performance Benchmarking**: Built-in performance testing tools
4. **Automated Testing**: Integration with CI/CD pipelines
5. **Cloud Deployment**: Support for cloud-based validator deployment

### **Community Contributions**

Surfpool is designed to be extensible and welcomes community contributions:

- **Plugin System**: Develop custom plugins for specific use cases
- **Tool Integrations**: Add support for new development tools
- **Performance Improvements**: Contribute performance optimizations
- **Documentation**: Help improve documentation and examples
- **Bug Reports**: Report and help fix bugs

## Support

### **Documentation**
- Comprehensive API documentation
- Tutorial videos and guides
- Example projects and templates
- Best practices guide

### **Community**
- Discord server for real-time support
- GitHub issues for bug reports and feature requests
- Community forums for discussions and knowledge sharing
- Regular office hours for direct support

### **Professional Support**
- Enterprise support packages available
- Custom development and consulting
- Training and workshops
- Priority bug fixes and feature requests

---

**Version**: 1.0  
**Created**: 2025-06-18  
**Last Updated**: 2025-06-18  
**Compatibility**: Solana 1.18+  
**License**: MIT

Surfpool is the foundation of local Solana development in SurfDesk, providing developers with the tools they need to build, test, and deploy Solana programs with confidence and efficiency.