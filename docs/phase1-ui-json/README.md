# Phase 1: UI → JSON Implementation Guide

## 🧩 Overview

This guide details the implementation of **Phase 1 (UI → JSON)** for the Solana Account Studio. The goal is to build a user interface that enables anyone to visually design a **Solana program structure** and automatically generate a **JSON schema** representation.

## 🎯 Core Objectives

1. **Visual Program Design**: Create an intuitive interface for designing Solana programs
2. **Real-time JSON Generation**: Automatically generate JSON schemas as users build programs
3. **No-Code Experience**: Enable program creation without writing code
4. **Immediate Feedback**: Show JSON output in real-time for instant validation
5. **Export Capabilities**: Allow users to export generated schemas for development

## 🏗️ Architecture Overview

### Data Flow
```
User Input → UI Components → State Management → JSON Generation → Preview Panel
```

### Component Structure
```
ProgramBuilder (Root)
├── ProgramInfoSection
├── AccountsSection
│   ├── AccountItem
│   └── FieldItem
├── InstructionsSection
│   ├── InstructionItem
│   └── ArgumentItem
└── JSONPreviewPanel
```

## 📋 User Capabilities

### 1. Program Information Management
- Define program name and version
- Set program metadata
- Configure program-specific settings

### 2. Account Definition
- Add multiple accounts with custom names
- Define fields for each account
- Select field types from comprehensive list:
  - `u8`, `u16`, `u32`, `u64` (unsigned integers)
  - `i8`, `i16`, `i32`, `i64` (signed integers)
  - `String` (text data)
  - `Pubkey` (Solana public keys)
  - `Vec<T>` (dynamic arrays)
  - `Option<T>` (optional fields)
  - `bool` (boolean values)
  - Custom struct types

### 3. Instruction Creation
- Create instruction definitions
- Select which accounts are used by each instruction
- Define instruction arguments with type validation
- Set account mutability and signer requirements

### 4. Real-time JSON Preview
- Live JSON schema updates
- Syntax highlighting
- Validation and error display
- Export/copy functionality

## 🎨 UI Layout Design

### Left Panel (60% width) - Controls
```
┌─────────────────────────────────────┐
│ Program Information                 │
│ ┌─────────────────────────────────┐ │
│ │ Program Name: [____________]    │ │
│ │ Version:      [____________]    │ │
│ └─────────────────────────────────┘ │
│                                     │
│ Accounts Section                    │
│ ┌─────────────────────────────────┐ │
│ │ [+ Add Account]                 │ │
│ │ ┌─ Account: Vault ──── [X]     │ │
│ │ │   [+ Add Field]              │ │
│ │ │   ┌─ Field: owner ── [X]     │ │
│ │ │   │   Type: [Pubkey ▼]       │ │
│ │ │   └─────────────────────────┘ │ │
│ │ └─────────────────────────────┘ │ │
│ └─────────────────────────────────┘ │
│                                     │
│ Instructions Section                │
│ ┌─────────────────────────────────┐ │
│ │ [+ Add Instruction]             │ │
│ │ ┌─ Instruction: deposit ─ [X]   │ │
│ │ │   Accounts: ☑ Vault ☑ User   │ │
│ │ │   Args: [+ Add Arg]          │ │
│ │ └─────────────────────────────┘ │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### Right Panel (40% width) - JSON Preview
```
┌─────────────────────────────────────┐
│ JSON Output                         │
│ ┌─────────────────────────────────┐ │
│ │ {                              │ │
│ │   "program": {                 │ │
│ │     "name": "vault_program",   │ │
│ │     "version": "0.1.0"         │ │
│ │   },                           │ │
│ │   "accounts": [                │ │
│ │     {                          │ │
│ │       "name": "Vault",         │ │
│ │       "fields": [              │ │
│ │         {                      │ │
│ │           "name": "owner",     │ │
│ │           "type": "Pubkey"     │ │
│ │         }                      │ │
│ │       ]                        │ │
│ │     }                          │ │
│ │   ]                            │ │
│ │ }                              │ │
│ └─────────────────────────────────┘ │
│ [📋 Copy JSON] [💾 Save] [📁 Load] │
└─────────────────────────────────────┘
```

## 🔧 Technical Implementation

### Core Data Structures

```rust
use serde::{Serialize, Deserialize};
use dioxus::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FieldType {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    String, Pubkey, Bool,
    Vec(Box<FieldType>),
    Option(Box<FieldType>),
    Custom(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Account {
    pub name: String,
    pub fields: Vec<Field>,
    pub discriminator: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstructionArg {
    pub name: String,
    pub arg_type: FieldType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Instruction {
    pub name: String,
    pub accounts: Vec<String>,
    pub args: Vec<InstructionArg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProgramSchema {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub accounts: Vec<Account>,
    pub instructions: Vec<Instruction>,
}
```

### State Management with Dioxus Signals

```rust
#[component]
pub fn ProgramBuilder() -> Element {
    let mut schema = use_signal(ProgramSchema::default);
    let mut selected_account = use_signal(|| None::<usize>);
    let mut selected_instruction = use_signal(|| None::<usize>);
    
    // Auto-save to localStorage
    use_effect(move || {
        if let Ok(json) = serde_json::to_string(&schema()) {
            // Save to localStorage
            web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .set_item("program_schema", &json)
                .unwrap();
        }
    });

    rsx! {
        div { class: "program-builder",
            // Main implementation here
        }
    }
}
```

## 🚀 Implementation Steps

### Step 1: Foundation (Week 1)
- [x] Define core data structures
- [x] Set up basic Dioxus component structure
- [x] Implement program name and version inputs
- [x] Create basic two-panel layout

### Step 2: Account Management (Week 1-2)
- [ ] Implement add/remove account functionality
- [ ] Create field management interface
- [ ] Add field type selection dropdown
- [ ] Implement field validation
- [ ] Add account reordering

### Step 3: Instruction Builder (Week 2)
- [ ] Create instruction creation interface
- [ ] Implement account selection for instructions
- [ ] Build argument definition system
- [ ] Add instruction validation

### Step 4: JSON Integration (Week 2-3)
- [x] Implement real-time JSON generation
- [x] Add JSON syntax highlighting
- [ ] Create JSON validation system
- [ ] Add export/copy functionality
- [ ] Implement import JSON feature

### Step 5: Advanced Features (Week 3)
- [ ] Add program templates
- [ ] Implement undo/redo functionality
- [ ] Create keyboard shortcuts
- [ ] Add dark/light theme support
- [ ] Implement project save/load

### Step 6: Polish & Testing (Week 3-4)
- [ ] Add comprehensive error handling
- [ ] Implement responsive design
- [ ] Add loading states and animations
- [ ] Create user onboarding flow
- [ ] Add accessibility features

## 🎯 Success Metrics

### User Experience
- **Time to First Program**: < 5 minutes
- **Learning Curve**: Minimal, intuitive interface
- **Error Rate**: < 5% for basic operations
- **User Satisfaction**: > 4.5/5

### Technical Performance
- **JSON Generation**: < 100ms
- **UI Responsiveness**: < 16ms frame time
- **Memory Usage**: < 100MB for complex programs
- **Auto-save Frequency**: Every change

### Feature Completion
- **Core Features**: 100% functional
- **Advanced Features**: 80% implemented
- **Edge Cases**: 90% covered
- **Error Handling**: 95% comprehensive

## 🔍 Testing Strategy

### Unit Tests
- Data structure serialization/deserialization
- Field type validation
- JSON generation accuracy
- State management correctness

### Integration Tests
- End-to-end program creation workflow
- Import/export functionality
- Auto-save behavior
- Cross-browser compatibility

### User Testing
- Usability testing with target users
- A/B testing for UI improvements
- Performance testing with large programs
- Accessibility testing

## 🛠️ Development Tools

### Required Dependencies
```toml
[dependencies]
dioxus = "0.6.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
web-sys = "0.3"
js-sys = "0.3"
wasm-bindgen = "0.2"
```

### Development Setup
```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Start development server
dx serve

# Build for production
dx build --release
```

## 📚 Next Steps

After completing Phase 1 UI → JSON, the next phases will include:

1. **Phase 2**: Code generation from JSON schemas
2. **Phase 3**: Integration with Solana RPC
3. **Phase 4**: Program deployment and testing
4. **Phase 5**: Advanced program features

## 🤝 Contributing

When contributing to Phase 1 implementation:

1. Follow the established component structure
2. Maintain consistency with existing UI patterns
3. Add comprehensive tests for new features
4. Update documentation for any API changes
5. Ensure accessibility standards are met

---

**Last Updated**: 2025-10-19  
**Phase**: 1 (UI → JSON)  
**Status**: 🚧 In Progress  
**Next Milestone**: Complete Account Management UI