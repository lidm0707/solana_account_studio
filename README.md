# 🏄‍♂️ SurfDesk - Solana Account Studio

<div align="center">

![SurfDesk Logo](https://img.shields.io/badge/SurfDesk-Solana%20Account%20Studio-blue?style=for-the-badge)
![Platform](https://img.shields.io/badge/Platform-Desktop%20%7C%20Web%20%7C%20Terminal%20%7C%20CLI-green?style=for-the-badge)
![Version](https://img.shields.io/badge/Version-0.1.0%20MVP-orange?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-purple?style=for-the-badge)

**A comprehensive multi-platform Solana account management studio**

[Quick Start](#-quick-start) • [Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Platforms](#-platforms)

</div>

## 🌟 About SurfDesk

SurfDesk is a **complete Solana account management solution** that runs on every platform. Whether you're a developer, trader, or Solana enthusiast, SurfDesk provides the tools you need to manage your accounts, build transactions, and interact with the Solana blockchain.

### 🎯 MVP Features

- ✅ **Multi-Account Management** - Create, import, and manage unlimited Solana accounts
- ✅ **Transaction Builder** - Create, sign, and send transactions with confidence
- ✅ **Real-time Balance Monitoring** - Track SOL balances across all accounts
- ✅ **Network Switching** - Seamlessly switch between Mainnet, Devnet, and Testnet
- ✅ **Cross-Platform** - Native apps for Desktop, Web, Terminal, and CLI
- ✅ **Local Validator Integration** - Built-in SurfPool support for local development

---

## 🚀 Quick Start

### Option 1: Desktop App (Recommended)

```bash
# Download and run the desktop application
./target/release/surfdesk-desktop
```

### Option 2: Web App

Open your browser and navigate to the web application:
```bash
# Serve the web application locally
./target/release/surfdesk-web
# Then visit http://localhost:8080
```

### Option 3: Terminal App

```bash
# Launch the terminal interface
./target/release/surfdesk-tui
```

### Option 4: CLI Tool

```bash
# Command-line interface for power users
./target/release/surfdesk-cli --help

# Check your account balance
./target/release/surfdesk-cli balance YOUR_PUBKEY

# Create a new account
./target/release/surfdesk-cli account create --label "My Account"
```

---

## 🛠️ Installation

### Prerequisites

- **Rust 1.70+** (for building from source)
- **Node.js 16+** (for web development)
- **SQLite 3** (included with application)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/your-org/surfdesk.git
cd surfdesk

# Build all platforms
cargo build --release --workspace

# Individual platform builds
cargo build --release --bin surfdesk-desktop
cargo build --release --bin surfdesk-web
cargo build --release --bin surfdesk-tui
cargo build --release --bin surfdesk-cli
```

### Pre-built Binaries

Download pre-built binaries for your platform from the [Releases page](https://github.com/your-org/surfdesk/releases).

---

## 📖 Usage Guide

### Desktop Application

1. **Launch the app** - Run `surfdesk-desktop`
2. **Create/Import Account** - Use the Account Manager to add your accounts
3. **View Balances** - Monitor real-time SOL balances
4. **Build Transactions** - Use the Transaction Builder interface
5. **Send Transactions** - Sign and broadcast to the network

### Web Application

1. **Open browser** - Navigate to your SurfDesk web instance
2. **Connect Wallet** - Import existing accounts or create new ones
3. **Manage Accounts** - View and organize your portfolio
4. **Transaction Interface** - Build and send transactions
5. **Network Settings** - Switch between Mainnet/Devnet/Testnet

### Terminal Interface (TUI)

```bash
# Launch terminal interface
./target/release/surfdesk-tui

# Keyboard shortcuts:
# Tab         - Switch between panels
# Enter       - Select/Confirm
# Esc         - Go back/Cancel
# q           - Quit application
# Ctrl+C      - Force quit
```

### Command-Line Interface (CLI)

#### Account Management

```bash
# Create new account
surfdesk-cli account create --label "Trading Account"

# Import existing account
surfdesk-cli account import --private-key "your_private_key"

# List all accounts
surfdesk-cli account list

# Get account info
surfdesk-cli account YOUR_PUBKEY

# Check balance
surfdesk-cli balance YOUR_PUBKEY
```

#### Transaction Operations

```bash
# Send SOL
surfdesk-cli send --from FROM_PUBKEY --to TO_PUBKEY --amount 1000000000

# Get transaction status
surfdesk-cli transaction YOUR_SIGNATURE

# Airdrop (devnet/testnet only)
surfdesk-cli airdrop YOUR_PUBKEY --amount 2000000000
```

#### Network Management

```bash
# Connect to custom RPC
surfdesk-cli connect --url https://api.mainnet-beta.solana.com

# Test connection
surfdesk-cli connect --url https://api.devnet.solana.com --test

# Switch networks
surfdesk-cli config set network devnet
```

#### Database Operations

```bash
# Initialize database
surfdesk-cli database init

# Check database status
surfdesk-cli database status

# Backup database
surfdesk-cli database backup --path /path/to/backup.sql

# Reset database (careful!)
surfdesk-cli database reset
```

---

## 🖥️ Platform Details

### Desktop Application
- **Native Performance** - Built with Rust and Dioxus
- **System Integration** - File dialogs, notifications, system tray
- **Offline Support** - Full functionality without internet
- **Auto-updates** - Built-in update mechanism

### Web Application  
- **Browser Compatible** - Works on all modern browsers
- **Responsive Design** - Mobile-friendly interface
- **PWA Support** - Install as a web app
- **Wallet Integration** - Compatible with popular wallets

### Terminal Interface (TUI)
- **Keyboard Driven** - Efficient navigation without mouse
- **Low Resource** - Runs on minimal system requirements
- **SSH Friendly** - Works over remote connections
- **Customizable** - Configurable themes and keybindings

### Command-Line Interface (CLI)
- **Scriptable** - Perfect for automation and scripts
- **Pipe Friendly** - Works with Unix pipelines
- **CI/CD Ready** - Integrates with development workflows
- **JSON Output** - Machine-readable output format

---

## ⚙️ Configuration

### Configuration File Location

| Platform | Config Path |
|----------|-------------|
| Linux | `~/.config/surfdesk/config.toml` |
| macOS | `~/Library/Application Support/SurfDesk/config.toml` |
| Windows | `%APPDATA%\SurfDesk\config.toml` |

### Default Configuration

```toml
[network]
default_rpc = "https://api.devnet.solana.com"
network = "devnet"  # mainnet, devnet, testnet

[database]
path = "~/.local/share/surfdesk/surfdesk.db"
backup_enabled = true
backup_interval = 24  # hours

[ui]
theme = "dark"
language = "en"
auto_refresh = true
refresh_interval = 30  # seconds

[logging]
level = "info"
file_path = "~/.local/share/surfdesk/logs/surfdesk.log"
max_file_size = "10MB"
max_files = 5
```

### Environment Variables

```bash
# Solana RPC URL
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"

# Database path
export SURFDESK_DB_PATH="/custom/path/to/database.db"

# Log level
export SURFDESK_LOG_LEVEL="debug"

# Network
export SURFDESK_NETWORK="mainnet"
```

---

## 🔧 Development

### Project Structure

```
surfdesk/
├── surfdesk-core/          # Shared library
│   ├── accounts/          # Account management
│   ├── transactions/      # Transaction building
│   ├── services/          # Core services
│   ├── components/        # UI components
│   ├── database/          # Data persistence
│   └── surfpool/          # Validator management
├── surfdesk-desktop/      # Desktop app
├── surfdesk-web/         # Web app
├── surfdesk-tui/         # Terminal app
├── surfdesk-cli/         # CLI tool
├── scripts/              # Automation scripts
└── docs/                 # Documentation
```

### Development Commands

```bash
# Install dependencies
cargo build

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Build with optimizations
cargo build --release

# Run specific platform
cargo run --bin surfdesk-desktop
cargo run --bin surfdesk-web
cargo run --bin surfdesk-tui
cargo run --bin surfdesk-cli
```

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Commit changes: `git commit -m 'Add amazing feature'`
6. Push to branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

---

## 🛡️ Security

### Private Key Management

- **Local Storage Only** - Private keys never leave your device
- **Encryption** - All sensitive data is encrypted at rest
- **Memory Safety** - Keys are cleared from memory after use
- **No Telemetry** - No data is sent to external servers

### Security Best Practices

1. **Never share private keys** - Keep them secret and secure
2. **Use strong passwords** - If using encrypted key storage
3. **Regular backups** - Backup your encrypted wallet files
4. **Verify transactions** - Always check transaction details before signing
5. **Keep software updated** - Use the latest version of SurfDesk

### Auditing

The codebase is designed with security in mind:
- Memory-safe Rust implementation
- No unsafe code blocks
- Regular security audits planned
- Open source for community review

---

## ❓ FAQ

### General Questions

**Q: Is SurfDesk free to use?**
A: Yes, SurfDesk is completely free and open source under the MIT license.

**Q: Can I use SurfDesk on multiple devices?**
A: Yes! SurfDesk runs on Desktop, Web, Terminal, and CLI platforms.

**Q: Does SurfDesk support hardware wallets?**
A: Hardware wallet support is planned for future releases.

### Technical Questions

**Q: What Solana networks are supported?**
A: Mainnet-beta, Devnet, and Testnet are fully supported.

**Q: Can I run my own local validator?**
A: Yes, SurfDesk integrates with SurfPool for local validator management.

**Q: How are private keys stored?**
A: Private keys are encrypted and stored locally on your device only.

### Troubleshooting

**Q: Desktop app won't start**
A: Ensure you have the latest system drivers and try running as administrator.

**Q: Connection issues to Solana network**
A: Check your internet connection and try switching RPC endpoints.

**Q: Transaction failed**
A: Verify account balances and network status. Check transaction fees.

---

## 📞 Support & Community

### Getting Help

- **Documentation**: [docs.surfdesk.dev](https://docs.surfdesk.dev)
- **Issues**: [GitHub Issues](https://github.com/your-org/surfdesk/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/surfdesk/discussions)
- **Discord**: [Join our Discord](https://discord.gg/surfdesk)

### Reporting Bugs

When reporting bugs, please include:
1. Platform (Desktop/Web/Terminal/CLI)
2. Operating system and version
3. Steps to reproduce
4. Expected vs actual behavior
5. Error messages or logs

### Feature Requests

We welcome feature requests! Please:
1. Check existing issues first
2. Provide detailed description
3. Explain use case and benefits
4. Consider implementation complexity

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Third-Party Licenses

SurfDesk uses the following open-source libraries:
- [Dioxus](https://dioxuslabs.com/) - MIT License
- [Solana SDK](https://solana.com/) - Apache 2.0 License
- [Diesel](https://diesel.rs/) - MIT or Apache 2.0 License
- [Tokio](https://tokio.rs/) - MIT License

---

## 🎉 Roadmap

### Version 0.2.0 (Planned)
- [ ] Hardware wallet support
- [ ] Mobile applications (iOS/Android)
- [ ] Advanced DeFi integrations
- [ ] Portfolio analytics
- [ ] Multi-signature support

### Version 0.3.0 (Future)
- [ ] NFT management
- [ ] Token swap integration
- [ ] Advanced transaction types
- [ ] Plugin system
- [ ] Enterprise features

---

<div align="center">

**Built with ❤️ by the SurfDesk Team**

[Website](https://surfdesk.dev) • [Twitter](https://twitter.com/surfdesk) • [Discord](https://discord.gg/surfdesk)

---

*SurfDesk v0.1.0 MVP - Your Gateway to Solana* 🏄‍♂️

</div>