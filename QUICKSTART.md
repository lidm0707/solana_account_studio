# 🚀 SurfDesk Quick Start Guide

Get up and running with SurfDesk in under 5 minutes!

## 📋 What You'll Need

- **SurfDesk binaries** (built or downloaded)
- **Solana account** (existing or new)
- **5 minutes** of your time ⏱️

---

## 🎯 Choose Your Platform

### 🖥️ Desktop App (Easiest)
```bash
# Launch the desktop application
./target/release/surfdesk-desktop
```
👉 **Best for**: Most users, visual interface, full features

### 🌐 Web App (No Installation)
```bash
# Start web server
./target/release/surfdesk-web
# Open browser to http://localhost:8080
```
👉 **Best for**: Quick access, any device with browser

### 💻 Terminal App (Power Users)
```bash
# Launch terminal interface
./target/release/surfdesk-tui
```
👉 **Best for**: SSH, remote servers, keyboard lovers

### ⌨️ CLI Tool (Developers)
```bash
# Command-line interface
./target/release/surfdesk-cli --help
```
👉 **Best for**: Automation, scripts, development

---

## 🔑 First Steps

### 1. Create Your First Account
```bash
# Using CLI (easiest for demo)
./target/release/surfdesk-cli account create --label "My First Account"

# You'll see output like:
# ✅ Account created successfully!
# 📋 Public Key: 11111111111111111111111111111111
# 🔐 Private Key: [SAVE THIS SECURELY]
# 📁 Saved to: ~/.local/share/surfdesk/accounts/
```

### 2. Check Your Balance
```bash
# Check SOL balance
./target/release/surfdesk-cli balance 11111111111111111111111111111111

# Output:
# 💰 Balance: 0.000000000 SOL
# 📍 Network: Devnet
```

### 3. Get Some Test SOL (Devnet Only)
```bash
# Request airdrop (2 SOL)
./target/release/surfdesk-cli airdrop 11111111111111111111111111111111 --amount 2000000000

# Output:
# 🎉 Airdrop successful! 2.0 SOL requested
# ⏳ Please wait a few seconds for confirmation...
```

### 4. Send Your First Transaction
```bash
# Send 0.1 SOL to yourself (for testing)
./target/release/surfdesk-cli send \
  --from 11111111111111111111111111111111 \
  --to 11111111111111111111111111111111 \
  --amount 100000000

# Output:
# 📤 Transaction sent successfully!
# 🔄 Signature: 5j7s8...
# 🔗 Explorer: https://solscan.io/tx/5j7s8...
```

---

## 🌐 Network Switching

### Switch to Mainnet (Real Money!)
```bash
./target/release/surfdesk-cli config set network mainnet
./target/release/surfdesk-cli connect --url https://api.mainnet-beta.solana.com
```

### Switch to Devnet (Test Money!)
```bash
./target/release/surfdesk-cli config set network devnet
./target/release/surfdesk-cli connect --url https://api.devnet.solana.com
```

### Switch to Testnet (Sandbox)
```bash
./target/release/surfdesk-cli config set network testnet
./target/release/surfdesk-cli connect --url https://api.testnet.solana.com
```

---

## 📱 Mobile Quick Access

### Web App on Phone
1. Start web server: `./target/release/surfdesk-web`
2. Find your computer's IP: `ip addr` (Linux) or `ifconfig` (Mac)
3. On phone, visit: `http://YOUR_IP:8080`
4. Add to home screen for app-like experience

---

## 🔧 Common Commands Cheat Sheet

```bash
# Account Management
./target/release/surfdesk-cli account list                           # List all accounts
./target/release/surfdesk-cli account create --label "Trading"       # New account
./target/release/surfdesk-cli account YOUR_PUBKEY                    # Account details

# Balance & Transactions
./target/release/surfdesk-cli balance YOUR_PUBKEY                    # Check balance
./target/release/surfdesk-cli send --from FROM --to TO --amount 1000 # Send SOL
./target/release/surfdesk-cli transaction SIGNATURE                  # TX status

# Network Operations
./target/release/surfdesk-cli connect --url RPC_URL --test           # Test connection
./target/release/surfdesk-cli airdrop YOUR_PUBKEY --amount 1000000000 # Get test SOL

# Configuration
./target/release/surfdesk-cli config show                            # Show settings
./target/release/surfdesk-cli config set network mainnet             # Change network
```

---

## 🚨 Important Security Notes

### ⚠️ NEVER Share Your Private Key
- Private keys give full control of your account
- Store them securely (password manager, encrypted file)
- Never paste private keys in chat/email/websites

### 🛡️ Best Practices
```bash
# Good: Use secure file storage
chmod 600 ~/.local/share/surfdesk/accounts/*

# Good: Use environment variables for sensitive data
export SURFDESK_RPC_URL="https://your-private-rpc.com"

# Bad: Don't commit keys to git
# Don't echo private keys in terminal
```

---

## 🆘 Troubleshooting

### "Connection Failed" Error
```bash
# Test RPC connection
./target/release/surfdesk-cli connect --url https://api.devnet.solana.com --test

# Try different RPC
./target/release/surfdesk-cli connect --url https://solana-api.projectserum.com --test
```

### "Account Not Found" Error
```bash
# Check if account exists on current network
./target/release/surfdesk-cli account YOUR_PUBKEY

# Try switching networks (account might be on mainnet)
./target/release/surfdesk-cli config set network mainnet
```

### "Transaction Failed" Error
```bash
# Check account balance first
./target/release/surfdesk-cli balance YOUR_PUBKEY

# Check network status
./target/release/surfdesk-cli connect --test

# Verify account format (should be 44 chars starting with 1)
```

---

## 🎯 What's Next?

### Explore Desktop App
```bash
./target/release/surfdesk-desktop
```
- Visual account management
- Transaction builder with UI
- Real-time balance updates
- Network switching with clicks

### Try Terminal Interface
```bash
./target/release/surfdesk-tui
```
- Keyboard navigation (Tab, Enter, Esc)
- Real-time updates
- Multiple account view
- Transaction history

### Build Your Own Tools
```bash
# Use CLI in scripts
#!/bin/bash
ACCOUNT=$(./target/release/surfdesk-cli account list | head -1)
BALANCE=$(./target/release/surfdesk-cli balance $ACCOUNT)
echo "Account: $ACCOUNT"
echo "Balance: $BALANCE"
```

---

## 📞 Need Help?

- **Full Documentation**: `README.md`
- **GitHub Issues**: Report bugs and request features
- **Community Discord**: chat with other users
- **Examples**: Check `examples/` directory

---

## 🎉 You're Ready!

Congratulations! 🎊 You now have:
- ✅ A working Solana account
- ✅ Test SOL in your wallet
- ✅ Successfully sent a transaction
- ✅ Knowledge of all SurfDesk platforms

**Start building your Solana journey with SurfDesk!** 🚀

---

*Last updated: SurfDesk v0.1.0 MVP*  
*Time to complete: 5 minutes*  
*Difficulty: Beginner-friendly* 🌟