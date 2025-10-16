# 📖 SurfDesk Platform Usage Guide

Detailed usage instructions for each SurfDesk platform.

---

## 🖥️ Desktop Application

### Launching the App
```bash
# Direct launch
./target/release/surfdesk-desktop

# With custom configuration
./target/release/surfdesk-desktop --config /path/to/config.toml

# Debug mode
./target/release/surfdesk-desktop --log-level debug
```

### User Interface Overview

#### Main Window Layout
```
┌─────────────────────────────────────────────────────────────┐
│ 🏄‍♂️ SurfDesk                    [─] [□] [×]               │
├─────────────────────────────────────────────────────────────┤
│ [Accounts] [Transactions] [Network] [Settings] [Help]      │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────────────────────────────────┐ │
│ │   Sidebar   │ │            Main Content               │ │
│ │             │ │                                       │ │
│ │ • Account 1 │ │  Account Details & Balance            │ │
│ │ • Account 2 │ │                                       │ │
│ │ • Account 3 │ │  ┌─────────────────────────────────┐  │ │
│ │             │ │  │      Transaction Builder        │  │ │
│ │ + Add New   │ │  │                                 │  │ │
│ │             │ │  │  From: [Account Dropdown]       │  │ │
│ │ ┌───────────┐ │ │  │  To:   [Input Field]          │  │ │
│ │ │ Network   │ │ │  │  Amount: [Input] SOL          │  │ │
│ │ │ Devnet    │ │ │  │                                 │  │ │
│ │ └───────────┘ │ │  └─────────────────────────────────┘  │ │
│ └─────────────┘ └─────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│ Status: Connected to Devnet | Balance: 2.5 SOL | [Sync]    │
└─────────────────────────────────────────────────────────────┘
```

#### Navigation
- **Tab Key**: Switch between sections
- **Arrow Keys**: Navigate within sections
- **Enter**: Select/Confirm action
- **Escape**: Cancel/Go back
- **Ctrl+N**: New account
- **Ctrl+S**: Send transaction
- **Ctrl+R**: Refresh balances

#### Account Management
1. **Create New Account**
   - Click "+" in sidebar or press `Ctrl+N`
   - Enter account label
   - Choose account type (Standard/Custom)
   - Save private key securely

2. **Import Existing Account**
   - Click "Import" button
   - Choose import method:
     - Private key (base58)
     - Keystore file
     - Seed phrase
   - Verify and confirm

3. **Account Actions**
   - Right-click account for context menu
   - Options: Rename, Export, Delete, View Details

#### Transaction Builder
1. **Basic Transfer**
   ```
   From: [Select Account]
   To: [Enter Public Key or Select from Contacts]
   Amount: [Enter SOL amount]
   Fee: [Auto-calculated]
   ──────────────────────────────────────
   [Preview] [Send Transaction]
   ```

2. **Advanced Options**
   - Memo field
   - Custom priority fee
   - Multiple recipients
   - Token transfers (coming soon)

#### Network Management
- **Network Switcher**: Top-right dropdown
- **Custom RPC**: Settings → Network → Add Custom RPC
- **Connection Status**: Bottom status bar
- **Latency Display**: Shows RPC response time

### File Operations
```
# Configuration location
Linux:   ~/.config/surfdesk/
macOS:   ~/Library/Application Support/SurfDesk/
Windows: %APPDATA%\SurfDesk\

# Data storage
~/.local/share/surfdesk/
├── accounts/          # Account files
├── database/          # SQLite database
├── logs/             # Application logs
└── cache/            # Cached data
```

---

## 🌐 Web Application

### Accessing the Web App
```bash
# Start local server
./target/release/surfdesk-web --port 8080

# Custom port
./target/release/surfdesk-web --port 3000

# Production mode
./target/release/surfdesk-web --production
```

### Browser Compatibility
- ✅ Chrome 90+
- ✅ Firefox 88+
- ✅ Safari 14+
- ✅ Edge 90+
- ✅ Mobile browsers

### Web Interface Features

#### Progressive Web App (PWA)
1. **Install to Home Screen**
   - Open in Chrome/Safari
   - Click "Install" icon in address bar
   - App appears in launcher/home screen

2. **Offline Support**
   - Basic account viewing works offline
   - Transaction queue for when online
   - Cached balance data

#### Responsive Design
```
Desktop (>1200px):    Full sidebar + main content
Tablet (768-1200px):  Collapsible sidebar
Mobile (<768px):      Bottom navigation + cards
```

#### Browser-Specific Features

**Chrome Extension Integration**
```javascript
// Access SurfDesk from browser console
window.surfdesk.getAccounts()
window.surfdesk.sendTransaction(tx)
window.surfdesk.getBalance(pubkey)
```

**MetaMask Compatibility**
```
Future: MetaMask-like wallet integration for Solana
```

### Web App Shortcuts
- **Ctrl+K**: Quick search accounts
- **Ctrl+/**: Show keyboard shortcuts
- **Ctrl+B**: Toggle sidebar
- **Ctrl+Shift+N**: New account
- **Ctrl+Shift+S**: Send transaction

---

## 💻 Terminal Interface (TUI)

### Launching TUI
```bash
# Basic launch
./target/release/surfdesk-tui

# With theme
./target/release/surfdesk-tui --theme dark

# Debug mode
./target/release/surfdesk-tui --debug

# Custom data directory
./target/release/surfdesk-tui --data-dir /custom/path
```

### TUI Layout
```
┌─ SurfDesk Terminal v0.1.0 ─────────────────────────────────────┐
│ Network: Devnet │ Accounts: 3 │ Balance: 2.5 SOL │ [Help: F1] │
├───────────────────────────────────────────────────────────────┤
│ ┌─ Accounts ──┐ ┌─ Details ─────────────────────────────────┐ │
│ │● Trading    │ │ Account: Trading                         │ │
│ │○ Savings    │ │ Pubkey: 11111111111111111111111111111111 │ │
│ │○ Main       │ │ Balance: 1.5 SOL                        │ │
│ │             │ │ Created: 2024-01-15                     │ │
│ │+ [New]      │ │                                         │ │
│ └─────────────┘ │ ┌─ Actions ─────────────────────────────┐ │ │
│                 │ │ [S] Send SOL                          │ │ │
│ ┌─ Recent Tx ──┐ │ │ [R] Receive                          │ │ │
│ │→ Send 0.1 SOL│ │ │ [A] Account Details                  │ │ │
│ │← Receive 2.0 │ │ │ [H] Transaction History              │ │ │
│ │→ Send 0.5 SOL│ │ │ [N] Network Settings                 │ │ │
│ │← Airdrop 2.0 │ │ └──────────────────────────────────────┘ │ │
│ └─────────────┘ │                                         │ │
│                 └─────────────────────────────────────────┘ │
├───────────────────────────────────────────────────────────────┤
│ Status: Connected │ Latency: 45ms │ [Q]uit │ [?]Help        │
└───────────────────────────────────────────────────────────────┘
```

### Keyboard Navigation

#### Global Shortcuts
```
F1          Show help
F2          Settings
F5          Refresh data
Tab         Switch between panels
Shift+Tab   Reverse panel switch
Enter       Select/Confirm
Escape      Go back/Cancel
Q or q      Quit application
```

#### Account Panel
```
↑/↓         Navigate accounts
Enter       View account details
N           New account
I           Import account
D           Delete account (with confirmation)
R           Rename account
```

#### Transaction Panel
```
↑/↓         Navigate transactions
Enter       View transaction details
S           Send SOL
A           Airdrop (devnet only)
H           History view
F           Filter transactions
```

#### Action Shortcuts
```
S           Send SOL dialog
R           Receive (show address)
A           Account settings
N           Network settings
C           Configuration
L           Logs viewer
```

### TUI Commands
```
# Command mode (press :)
:help        Show help
:quit        Exit application
:refresh     Refresh all data
:network     Switch network
:theme       Change theme
:clear       Clear screen
:version     Show version info
```

### TUI Themes
```bash
# Available themes
./target/release/surfdesk-tui --theme dark      # Default dark theme
./target/release/surfdesk-tui --theme light     # Light theme
./target/release/surfdesk-tui --theme blue      # Blue theme
./target/release/surfdesk-tui --theme matrix    # Matrix-style theme

# Custom theme file
./target/release/surfdesk-tui --theme /path/to/theme.toml
```

---

## ⌨️ Command-Line Interface (CLI)

### Basic CLI Usage
```bash
# Show help
./target/release/surfdesk-cli --help

# Global options
./target/release/surfdesk-cli --verbose --output json balance ACCOUNT
./target/release/surfdesk-cli --config /path/to/config.toml account list
```

### Output Formats
```bash
# Table format (default)
./target/release/surfdesk-cli account list
┌──────────────────────────────────────┬──────────┬─────────┐
│ Account                               │ Balance  │ Network │
├──────────────────────────────────────┼──────────┼─────────┤
│ 11111111111111111111111111111111     │ 1.5 SOL  │ Devnet  │
│ 22222222222222222222222222222222     │ 0.8 SOL  │ Devnet  │
└──────────────────────────────────────┴──────────┴─────────┘

# JSON format
./target/release/surfdesk-cli --output json account list
{
  "accounts": [
    {
      "pubkey": "11111111111111111111111111111111",
      "balance": 1500000000,
      "network": "devnet",
      "label": "Trading"
    }
  ]
}

# YAML format
./target/release/surfdesk-cli --output yaml account list
accounts:
  - pubkey: "11111111111111111111111111111111"
    balance: 1500000000
    network: "devnet"
    label: "Trading"
```

### Command Reference

#### Account Commands
```bash
# List accounts
surfdesk-cli account list [--output table|json|yaml]

# Create account
surfdesk-cli account create [--label LABEL] [--type TYPE]

# Import account
surfdesk-cli account import --private-key KEY [--label LABEL]
surfdesk-cli account import --keyfile PATH [--label LABEL]
surfdesk-cli account import --seed-phrase "word1 word2..." [--label LABEL]

# Account details
surfdesk-cli account PUBKEY [--output table|json|yaml]

# Export account
surfdesk-cli account export PUBKEY --format json --output /path/to/file

# Delete account
surfdesk-cli account delete PUBKEY [--confirm]
```

#### Balance Commands
```bash
# Check balance
surfdesk-cli balance PUBKEY [--output table|json|yaml]

# Multiple balances
surfdesk-cli balance PUBKEY1 PUBKEY2 PUBKEY3

# All account balances
surfdesk-cli balance --all
```

#### Transaction Commands
```bash
# Send SOL
surfdesk-cli send --from FROM_PUBKEY --to TO_PUBKEY --amount LAMPORTS
surfdesk-cli send --from FROM --to TO --amount 1.5 --unit sol

# Send with memo
surfdesk-cli send --from FROM --to TO --amount 1000000000 --memo "Payment"

# Batch send
surfdesk-cli send-batch --file transactions.json

# Transaction status
surfdesk-cli transaction SIGNATURE [--output table|json|yaml]

# Transaction history
surfdesk-cli transaction history --account PUBKEY [--limit 50]

# Confirm transaction
surfdesk-cli transaction confirm SIGNATURE
```

#### Network Commands
```bash
# Connect to RPC
surfdesk-cli connect --url RPC_URL [--test]

# Test connection
surfdesk-cli connect --test

# Network info
surfdesk-cli network info

# Switch networks
surfdesk-cli network mainnet
surfdesk-cli network devnet
surfdesk-cli network testnet
```

#### Configuration Commands
```bash
# Show configuration
surfdesk-cli config show [--output table|json|yaml]

# Get configuration value
surfdesk-cli config get network
surfdesk-cli config get rpc_url

# Set configuration value
surfdesk-cli config set network mainnet
surfdesk-cli config set rpc_url "https://api.mainnet-beta.solana.com"

# Reset configuration
surfdesk-cli config reset [--confirm]
```

#### Database Commands
```bash
# Initialize database
surfdesk-cli database init

# Database status
surfdesk-cli database status

# Backup database
surfdesk-cli database backup --path /path/to/backup.sql

# Restore database
surfdesk-cli database restore --path /path/to/backup.sql

# Reset database
surfdesk-cli database reset [--confirm]

# Database migration
surfdesk-cli database migrate
```

#### Development Commands
```bash
# Generate test data
surfdesk-cli dev generate-test-data --accounts 10 --transactions 50

# Run benchmarks
surfdesk-cli dev benchmark --type rpc --iterations 100

# Validate project
surfdesk-cli dev validate

# Generate documentation
surfdesk-cli dev docs
```

### Environment Variables
```bash
# Configuration
export SURFDESK_CONFIG_DIR="/custom/config/path"
export SURFDESK_DATA_DIR="/custom/data/path"
export SURFDESK_LOG_LEVEL="debug|info|warn|error"

# Network
export SURFDESK_RPC_URL="https://api.mainnet-beta.solana.com"
export SURFDESK_NETWORK="mainnet|devnet|testnet"

# Output
export SURFDESK_OUTPUT_FORMAT="table|json|yaml"
export SURFDESK_NO_COLOR="true"
export SURFDESK_QUIET="true"
```

### Shell Integration

#### Bash Completion
```bash
# Enable completion
source <(surfdesk-cli completion bash)

# Add to .bashrc
echo 'source <(surfdesk-cli completion bash)' >> ~/.bashrc
```

#### Fish Completion
```bash
# Enable completion
surfdesk-cli completion fish | source

# Add to config.fish
surfdesk-cli completion fish > ~/.config/fish/completions/surfdesk-cli.fish
```

#### PowerShell Completion
```bash
# Enable completion
surfdesk-cli completion powershell | Out-String | Invoke-Expression

# Add to profile
surfdesk-cli completion powershell >> $PROFILE
```

### Scripting Examples

#### Balance Checker Script
```bash
#!/bin/bash
# check-balances.sh

ACCOUNTS=$(surfdesk-cli account list --output json | jq -r '.accounts[].pubkey')

echo "=== Balance Report ==="
for account in $ACCOUNTS; do
    balance=$(surfdesk-cli balance "$account" --output json | jq -r '.balance')
    sol=$(echo "scale=4; $balance / 1000000000" | bc)
    echo "$account: $sol SOL"
done
```

#### Transaction Monitor
```bash
#!/bin/bash
# monitor-tx.sh

SIGNATURE=$1
if [ -z "$SIGNATURE" ]; then
    echo "Usage: $0 <signature>"
    exit 1
fi

while true; do
    status=$(surfdesk-cli transaction "$SIGNATURE" --output json | jq -r '.status')
    echo "$(date): Transaction status: $status"
    
    if [ "$status" != "pending" ]; then
        break
    fi
    
    sleep 5
done

echo "Transaction final status: $status"
```

---

## 🔧 Advanced Configuration

### Platform-Specific Settings

#### Desktop Configuration
```toml
[desktop]
theme = "dark"
window_size = [1200, 800]
window_position = [100, 100]
always_on_top = false
minimize_to_tray = true
auto_start = false

[desktop.shortcuts]
new_account = "Ctrl+N"
send_transaction = "Ctrl+S"
refresh = "F5"
```

#### Web Configuration
```toml
[web]
port = 8080
host = "127.0.0.1"
ssl_enabled = false
ssl_cert_path = ""
ssl_key_path = ""
max_connections = 100
enable_cors = true

[web.pwa]
enabled = true
offline_cache = true
cache_duration = 3600
```

#### Terminal Configuration
```toml
[terminal]
theme = "dark"
refresh_rate = 1000  # milliseconds
enable_mouse = true
enable_unicode = true
scrollback_lines = 1000

[terminal.colors]
foreground = "#ffffff"
background = "#000000"
primary = "#00ff00"
secondary = "#ffff00"
error = "#ff0000"
```

#### CLI Configuration
```toml
[cli]
default_output = "table"
enable_colors = true
confirm_destructive = true
history_file = "~/.surfdesk_history"
pager = "less"

[cli.aliases]
bal = "balance"
tx = "transaction"
acct = "account"
net = "network"
```

### Cross-Platform Integration

#### Data Synchronization
```bash
# Sync accounts between platforms
surfdesk-cli account export --all --format json > accounts.json
surfdesk-desktop --import-accounts accounts.json
surfdesk-web --import-accounts accounts.json
```

#### Shared Configuration
```bash
# Use same config across platforms
export SURFDESK_CONFIG_DIR="~/Dropbox/surfdesk"
surfdesk-cli account list
surfdesk-desktop
surfdesk-tui
```

---

## 🚀 Performance Tips

### Desktop Optimization
- Use SSD for database storage
- Increase memory allocation for large account sets
- Disable animations on slower systems

### Web Optimization
- Enable browser caching
- Use WebSocket connections for real-time updates
- Compress large transaction data

### Terminal Optimization
- Reduce refresh rate for slow connections
- Disable mouse support in SSH sessions
- Use minimal themes for performance

### CLI Optimization
- Use JSON output for scripting
- Batch operations when possible
- Cache RPC responses locally

---

*This guide covers all SurfDesk platforms. For more specific help, use the built-in help features or check the main README.md file.*