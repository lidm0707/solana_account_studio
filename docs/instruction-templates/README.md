# Instruction Templates - Block UI for Common Operations

## 🎯 Overview

The instruction templates feature provides a block-based UI for quickly adding common Solana operations to your programs. Instead of manually creating instructions from scratch, users can select from pre-built templates for operations like **swap**, **transfer**, **add liquidity**, and more.

## 🚀 Features

### Template Categories
- **Token Operations**: Swap, Transfer
- **Liquidity Operations**: Add Liquidity
- **Account Management**: Initialize Account
- **Data Operations**: Store, Update, Delete Data (coming soon)
- **Validation**: Validate Owner, Check Balance (coming soon)

### Key Benefits
- **🏗️ Visual Building Blocks**: Drag-and-drop style interface
- **⚡ Rapid Development**: Add complex instructions in one click
- **📋 Pre-configured**: Accounts and arguments are automatically set up
- **🎨 Category Organization**: Templates grouped by functionality
- **🔄 Instant Integration**: Templates immediately appear in your program structure

## 📋 Available Templates

### Token Operations

#### 🔁 Swap
- **Description**: Swap tokens between accounts
- **Required Accounts**: `user_account`, `pool_account`, `mint_a`, `mint_b`, `authority`
- **Arguments**: 
  - `amount_in: u64` - Amount of input tokens
  - `minimum_amount_out: u64` - Minimum amount of output tokens expected
- **Use Case**: DEX operations, token exchanges

#### 💸 Transfer
- **Description**: Transfer tokens or SOL between accounts
- **Required Accounts**: `from_account`, `to_account`, `authority`
- **Arguments**: 
  - `amount: u64` - Amount to transfer
- **Use Case**: Payment systems, token movements

### Liquidity Operations

#### 💧 Add Liquidity
- **Description**: Add liquidity to a pool
- **Required Accounts**: `user_account`, `pool_account`, `mint_a`, `mint_b`, `authority`
- **Arguments**: 
  - `amount_a: u64` - Amount of first token
  - `amount_b: u64` - Amount of second token
- **Use Case**: Liquidity pools, AMM operations

### Account Management

#### 🆕 Initialize Account
- **Description**: Initialize a new account
- **Required Accounts**: `new_account`, `authority`
- **Arguments**: None
- **Use Case**: Setting up user accounts, program accounts

## 🎨 How to Use

### Step 1: Open Template Selector
1. Navigate to the Program Builder
2. In the Instructions section, click the **"📦 Templates"** button
3. A modal will appear showing all available templates

### Step 2: Select a Template
1. Browse templates by category
2. Click on any template card to add it to your program
3. The template will automatically be added to your instructions list

### Step 3: Customize (Optional)
1. After adding a template, you can:
   - Rename the instruction
   - Modify account names
   - Adjust argument types (if needed)
   - Add custom arguments

### Step 4: Review JSON Output
1. Watch the JSON panel update in real-time
2. Verify the instruction structure matches your requirements
3. Copy the JSON schema for development use

## 🔧 Technical Implementation

### Template Structure
Each template follows this structure:
```rust
pub struct InstructionTemplate {
    pub id: String,                    // Unique identifier
    pub name: String,                  // Display name
    pub description: String,           // Human-readable description
    pub required_accounts: Vec<String>, // Required account names
    pub optional_accounts: Vec<String>, // Optional account names
    pub args: Vec<InstructionArg>,     // Argument definitions
    pub category: String,              // Category for grouping
}
```

### JSON Output Example
When you add a "Swap" template, it generates:
```json
{
  "name": "Swap",
  "accounts": [
    "user_account",
    "pool_account", 
    "mint_a",
    "mint_b",
    "authority"
  ],
  "args": [
    {
      "name": "amount_in",
      "type": "U64"
    },
    {
      "name": "minimum_amount_out", 
      "type": "U64"
    }
  ]
}
```

## 🚧 Coming Soon

### Additional Templates
- **Close Account**: Close accounts and reclaim lamports
- **Store Data**: Store data in program accounts
- **Update Data**: Modify existing data
- **Delete Data**: Remove data from accounts
- **Validate Owner**: Verify account ownership
- **Check Balance**: Query account balances
- **Verify Signature**: Cryptographic signature verification

### Enhanced Features
- **Custom Templates**: Create your own reusable templates
- **Template Library**: Share and import templates
- **Smart Suggestions**: AI-powered template recommendations
- **Visual Connections**: Draw connections between instructions
- **Batch Operations**: Add multiple templates at once

## 🎯 Best Practices

### When to Use Templates
- ✅ **Common Operations**: For standard DeFi operations like swaps and transfers
- ✅ **Rapid Prototyping**: Quickly build program structure
- ✅ **Learning**: Understand typical account and argument patterns
- ✅ **Consistency**: Maintain uniform instruction patterns across programs

### When to Customize
- 🔧 **Unique Logic**: Custom business logic not covered by templates
- 🔧 **Special Requirements**: Specific account structures or arguments
- 🔧 **Performance Optimization**: Fine-tuned instruction implementations
- 🔧 **Integration**: Connecting with external systems or protocols

### Customization Tips
1. **Rename Meaningfully**: Change template names to match your use case
2. **Account Mapping**: Map template accounts to your actual account structure
3. **Argument Types**: Adjust argument types based on your specific needs
4. **Validation**: Add custom validation logic in the final implementation

## 🔄 Integration with Development Workflow

### Phase 1: Design (Current)
- Use templates to quickly prototype program structure
- Generate JSON schemas for documentation
- Share program designs with team members

### Phase 2: Code Generation (Future)
- Convert template-based JSON to actual Anchor code
- Generate Rust implementations automatically
- Include proper error handling and validation

### Phase 3: Testing & Deployment (Future)
- Generate test cases from template structures
- Deploy to testnet for validation
- Monitor performance and optimize

## 🤝 Contributing Templates

We welcome contributions for new instruction templates! To add a template:

1. **Define the Template**: Create a new `InstructionTemplate` instance
2. **Add to Categories**: Place it in the appropriate category
3. **Document**: Include clear descriptions and use cases
4. **Test**: Verify it generates correct JSON output
5. **Submit**: Create a pull request with your template

### Template Guidelines
- 📝 **Clear Descriptions**: Explain what the template does
- 🏗️ **Standard Accounts**: Use common account naming conventions
- 🎯 **Specific Use Cases**: Provide clear usage scenarios
- 🧪 **Test Coverage**: Include example JSON outputs
- 📚 **Documentation**: Explain parameters and requirements

## 📚 Examples

### Example 1: Simple DEX Program
```rust
// Using templates to build a basic DEX
Program: "SimpleDEX"
Accounts:
  - Pool: { authority: Pubkey, token_a_balance: U64, token_b_balance: U64 }
  - User: { owner: Pubkey, token_a_balance: U64, token_b_balance: U64 }

Instructions:
  - Swap (Template): Swap tokens between user and pool
  - Add Liquidity (Template): Add tokens to pool
  - Initialize Account (Template): Set up user accounts
```

### Example 2: Token Staking Program
```rust
// Combining templates with custom instructions
Program: "StakeProtocol"
Accounts:
  - StakePool: { authority: Pubkey, total_staked: U64, reward_rate: U64 }
  - UserStake: { owner: Pubkey, amount: U64, rewards_earned: U64 }

Instructions:
  - Initialize Account (Template): Set up stake pool
  - Transfer (Template): Transfer tokens to stake
  - Stake (Custom): Custom staking logic
  - Unstake (Custom): Custom unstaking logic
```

## 🔍 Troubleshooting

### Common Issues

**Template Not Appearing**
- Check that the server is running
- Refresh the page and try again
- Verify template data is correctly formatted

**JSON Output Incorrect**
- Review the template structure in the code
- Check account names and argument types
- Verify the template is properly integrated

**Template Modal Not Closing**
- Click the × button in the top-right corner
- Try refreshing the page
- Check browser console for errors

**Performance Issues**
- Large numbers of templates may slow loading
- Consider using specific categories to filter
- Report performance issues for optimization

### Getting Help
- 📖 **Documentation**: Check this guide and the main README
- 🐛 **Issues**: Report bugs on GitHub
- 💬 **Discussions**: Join our community discussions
- 📧 **Support**: Contact the development team

---

## 🎉 Summary

Instruction templates provide a powerful, visual way to build Solana programs quickly. By combining pre-built templates with custom instructions, you can rapidly prototype and develop complex blockchain applications while maintaining the flexibility to customize every aspect of your program.

**Next Steps**:
1. Try the current templates in the Program Builder
2. Customize templates to fit your specific use cases
3. Contribute new templates for the community
4. Stay tuned for advanced features and AI-powered suggestions

**Happy Building! 🚀**