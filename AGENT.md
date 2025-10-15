# SurfDesk AI Agent Implementation Guide

## 🤖 Agent Role & Responsibilities

You are an expert software engineer tasked with implementing SurfDesk, a comprehensive **cross-platform Solana account studio** built with **Dioxus 0.6+**. Your role is to follow the ROADMAP.md step-by-step, implementing each component with high quality, testing thoroughly, and maintaining excellent documentation across all platforms (desktop, web, and terminal).

## 🎯 Core Principles

### **Quality First**
- Write clean, maintainable, and well-documented Rust code
- Follow Dioxus 0.6+ best practices and idiomatic patterns
- Ensure cross-platform compatibility and consistency
- Test thoroughly at unit, integration, and platform-specific levels
- Maintain >90% code coverage across all components

### **Multi-Platform Excellence**
- Ensure feature parity across desktop, web, and terminal platforms
- Leverage platform-specific capabilities while maintaining shared code
- Implement responsive design for web, native integrations for desktop
- Optimize performance for each platform's constraints
- Provide consistent user experience across all platforms

### **Incremental Progress**
- Complete one step fully before moving to the next
- Implement cross-platform features simultaneously where possible
- Commit frequently with meaningful, platform-aware messages
- Update step status immediately after completion
- Never skip steps or cut corners across platforms

### **Documentation Driven**
- Keep README and inline docs current for all platforms
- Update ROADMAP.md status as you progress
- Document platform-specific considerations and trade-offs
- Provide clear examples for all APIs and components
- Create platform-specific usage guides

## 📋 Step Implementation Workflow

### **Step Template**

For each step in ROADMAP.md, follow this exact workflow:

#### **1. Preparation Phase**
```bash
# Read the step requirements carefully
cat ROADMAP.md | grep -A 50 "Step [X].[Y]:"

# Create step-specific branch for all platforms
git checkout -b step-[X].[Y]-[step-name]

# Review cross-platform dependencies from previous steps
git log --oneline -10

# Verify platform targets are configured
cargo check --target x86_64-unknown-linux-gnu
cargo check --target wasm32-unknown-unknown
```

#### **2. Cross-Platform Implementation Phase**
```bash
# Create shared implementation directory structure
mkdir -p surfdesk-core/src/[component]/[subcomponent]

# Implement shared business logic
# Write platform-agnostic code first
# Then implement platform-specific adapters

# Create platform-specific implementations
# Desktop: surfdesk-desktop/src/
# Web: surfdesk-web/src/
# Terminal: surfdesk-tui/src/

# Ensure all platforms can build successfully
cargo build --release --bin surfdesk-desktop
cargo build --release --bin surfdesk-web
cargo build --release --bin surfdesk-tui
```

#### **3. Cross-Platform Testing Phase**
```bash
# Run shared unit tests
cargo test --lib

# Run platform-specific tests
cargo test --bin surfdesk-desktop
cargo test --bin surfdesk-web
cargo test --bin surfdesk-tui

# Run integration tests
cargo test --test integration

# Manual testing verification for all platforms
# - Desktop: Native UI testing, file system integration
# - Web: Browser testing, responsive design, WebAssembly functionality
# - Terminal: TUI testing, keyboard navigation, low resource usage

# Cross-platform feature parity verification
# - Test all acceptance criteria on each platform
# - Verify platform-specific optimizations work correctly
# - Check error handling and user feedback consistency
```

#### **4. Documentation Phase**
```bash
# Update shared documentation
# Add platform-specific sections to README files
# Update API documentation with platform considerations
# Create usage examples for each platform

# Document platform-specific features and limitations
# Add screenshots for desktop and web interfaces
# Create terminal interface documentation

# Update inline documentation
# Add context documentation files for new modules
# Document architectural decisions and platform trade-offs
```

#### **5. Commit & Status Update**
```bash
# Stage all changes across platforms
git add .

# Commit with detailed, platform-aware message
git commit -m "feat: implement Step [X].[Y] - [Step Name]

Multi-platform implementation:
- Shared business logic in surfdesk-core
- Desktop-specific features in surfdesk-desktop
- Web-specific features in surfdesk-web
- Terminal-specific features in surfdesk-tui

Cross-platform deliverables:
✅ [Deliverable 1 description]
✅ [Deliverable 2 description] 
✅ [Deliverable N description]

Platform-specific features:
✅ Desktop: [Desktop-specific feature]
✅ Web: [Web-specific feature]
✅ Terminal: [Terminal-specific feature]

Acceptance Criteria:
✅ [Criteria 1] - All platforms
✅ [Criteria 2] - All platforms
✅ [Criteria N] - All platforms

Tests:
✅ Unit tests passing (cargo test --lib)
✅ Integration tests passing (cargo test --test integration)
✅ Desktop tests passing (cargo test --bin surfdesk-desktop)
✅ Web tests passing (cargo test --bin surfdesk-web)
✅ Terminal tests passing (cargo test --bin surfdesk-tui)
✅ Manual testing verified on all platforms

Cross-platform verification:
✅ Feature parity confirmed
✅ Platform optimizations working
✅ Performance within acceptable limits

Closes #step-[X].[Y]"
```

#### **6. Status Update**
```bash
# Update ROADMAP.md step status
# Change ⏳ Not Started to ✅ Complete
# Add completion timestamp and platform notes
# Note any platform-specific issues or deviations
```

## 🔄 Multi-Platform Implementation Guide

### **Phase 1: Foundation & Dioxus 0.6+ Setup**

#### **Step 1.1: Dioxus Project Setup & Multi-Platform Configuration**
**Status**: ⏳ Not Started → 🔄 In Progress → ✅ Complete

**Implementation Checklist**:
- [ ] Initialize Cargo workspace with multiple binary targets
- [ ] Configure Dioxus 0.6+ for desktop, web, and terminal platforms
- [ ] Set up cross-compilation targets and build scripts
- [ ] Create platform abstraction layer
- [ ] Implement shared state management with Dioxus signals
- [ ] Set up cross-platform development environment

**Commands to Execute**:
```bash
# Initialize multi-platform Cargo workspace
cargo new --lib surfdesk-core
cargo new surfdesk-desktop --bin
cargo new surfdesk-web --bin  
cargo new surfdesk-tui --bin
cargo new surfdesk-cli --bin

# Configure workspace root Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "surfdesk-core",
    "surfdesk-desktop", 
    "surfdesk-web",
    "surfdesk-tui",
    "surfdesk-cli"
]
resolver = "2"

[workspace.dependencies]
dioxus = { version = "0.6", features = ["desktop", "web"] }
dioxus-router = "0.6"
dioxus-signals = "0.6"
solana-sdk = "1.18"
solana-client = "1.18"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
EOF

# Add Dioxus and platform dependencies
cd surfdesk-core
cargo add dioxus dioxus-router dioxus-signals
cargo add solana-sdk solana-client tokio serde serde_json

cd ../surfdesk-desktop
cargo add dioxus --features desktop
cargo add surfdesk-core --path ../surfdesk-core

cd ../surfdesk-web  
cargo add dioxus --features web
cargo add surfdesk-core --path ../surfdesk-core
cargo add gloo-web console_log log

cd ../surfdesk-tui
cargo add crossterm ratatui
cargo add surfdesk-core --path ../surfdesk-core

# Set up build scripts and platform configuration
mkdir -p scripts
echo '#!/bin/bash
cargo build --release --bin surfdesk-desktop
cargo build --release --bin surfdesk-web  
cargo build --release --bin surfdesk-tui
cargo build --release --bin surfdesk-cli
' > scripts/build-all.sh

chmod +x scripts/build-all.sh
```

**Expected Files Created**:
```
surfdesk/
├── Cargo.toml (workspace root)
├── surfdesk-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── app.rs
│       ├── state.rs
│       ├── services/
│       ├── components/
│       └── platform/
├── surfdesk-desktop/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── surfdesk-web/
│   ├── Cargo.toml
│   ├── src/
│       │   └── main.rs
│   └── index.html
├── surfdesk-tui/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── surfdesk-cli/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── scripts/
│   └── build-all.sh
├── .gitignore
├── rust-toolchain.toml
└── README.md
```

**Acceptance Criteria Verification**:
- [ ] All platforms build successfully without errors
- [ ] Basic Dioxus app renders on all platforms
- [ ] Cross-compilation targets configured correctly
- [ ] Platform detection and adaptation works
- [ ] Shared state management functions across platforms
- [ ] Development environment supports hot reload where applicable

**Testing Commands**:
```bash
# Build all platforms
./scripts/build-all.sh

# Test individual platforms
cargo run --bin surfdesk-desktop
cargo run --bin surfdesk-web
cargo run --bin surfdesk-tui

# Run tests
cargo test --lib
cargo test --bin surfdesk-desktop
cargo test --bin surfdesk-web
cargo test --bin surfdesk-tui
```

**Commit Message Template**:
```
feat: implement Step 1.1 - Dioxus 0.6+ Multi-Platform Project Setup

Multi-platform workspace initialization:
- Cargo workspace with desktop, web, and terminal targets
- Dioxus 0.6+ configuration for all platforms
- Cross-compilation support and build automation
- Platform abstraction layer foundation
- Shared state management with Dioxus signals

Core Components:
✅ surfdesk-core - Shared business logic and components
✅ surfdesk-desktop - Native desktop application
✅ surfdesk-web - WebAssembly web application  
✅ surfdesk-tui - Terminal interface
✅ surfdesk-cli - Headless CLI for automation

Platform Features:
✅ Desktop: Native window management and OS integration
✅ Web: Responsive design and browser compatibility
✅ Terminal: Keyboard-first interface and low resource usage
✅ CLI: Headless operation for CI/CD integration

Acceptance Criteria:
✅ All platforms build successfully without errors
✅ Basic Dioxus app renders on all platforms
✅ Cross-compilation targets configured correctly
✅ Platform detection and adaptation works
✅ Shared state management functions across platforms
✅ Development environment supports hot reload where applicable

Tests:
✅ Build tests passing (./scripts/build-all.sh)
✅ Desktop app running (cargo run --bin surfdesk-desktop)
✅ Web app serving (cargo run --bin surfdesk-web)
✅ Terminal interface working (cargo run --bin surfdesk-tui)
✅ Unit tests passing (cargo test --lib)
✅ Platform-specific tests passing

Cross-platform verification:
✅ Feature parity confirmed across all targets
✅ Platform-specific optimizations implemented
✅ Performance within acceptable limits
✅ Responsive design working for web target

Closes #step-1.1
```

---

### **Platform-Specific Implementation Guidelines**

#### **Desktop Platform (surfdesk-desktop)**
- Use Dioxus desktop renderer with native window management
- Implement native file dialogs, menus, and system tray integration
- Leverage OS-specific features (auto-updater, file associations)
- Optimize for desktop performance and resource usage
- Provide multi-monitor support and detachable panels

#### **Web Platform (surfdesk-web)**
- Use Dioxus web renderer with WebAssembly compilation
- Implement responsive design for mobile and desktop browsers
- Leverage browser storage (IndexedDB, localStorage) for persistence
- Provide PWA capabilities and offline support
- Optimize bundle size and loading performance

#### **Terminal Platform (surfdesk-tui)**
- Implement custom TUI renderer using crossterm and ratatui
- Focus on keyboard-first interaction and efficiency
- Optimize for low resource usage and server environments
- Provide rich ASCII art interface with colors and formatting
- Ensure compatibility with SSH and remote sessions

#### **CLI Platform (surfdesk-cli)**
- Headless operation for CI/CD and automation
- Focus on scriptability and programmatic usage
- Provide comprehensive command-line interface
- Support configuration files and environment variables
- Generate reports in multiple formats (JSON, HTML, JUnit)

---

## 🔧 Development Guidelines

### **Code Quality Standards**
- Follow Rust idioms and Dioxus 0.6+ best practices
- Use meaningful variable and function names with platform context
- Keep functions small and focused with clear responsibilities
- Write comprehensive documentation for all public APIs
- Handle errors gracefully with platform-specific considerations
- Ensure async/await usage is consistent and efficient

### **Cross-Platform Standards**
- Shared code should be platform-agnostic where possible
- Platform-specific code should be isolated in adapter modules
- Use feature flags to conditionally compile platform-specific code
- Implement consistent interfaces across all platforms
- Ensure data structures are serializable for cross-platform communication
- Test feature parity thoroughly across all platforms

### **Testing Standards**
- Write tests for all shared APIs with platform mocking
- Use property-based testing for core business logic
- Mock external dependencies for consistent testing
- Test both success and failure paths across platforms
- Maintain >90% code coverage across all components
- Include platform-specific integration tests

### **Documentation Standards**
- Document all public APIs with platform considerations
- Provide usage examples for each platform
- Include architectural decision records for trade-offs
- Keep README files current with platform-specific instructions
- Document troubleshooting procedures for each platform
- Create developer onboarding guides for multi-platform development

### **Commit Standards**
- Use conventional commit messages with platform context
- Commit frequently with meaningful messages
- Include platform-specific changes and considerations
- Reference relevant issue numbers and platform tickets
- Keep commits focused and atomic per platform

---

## 🚀 Getting Started

### **Initial Setup**
1. **Clone and configure repository**:
   ```bash
   git clone https://github.com/your-org/surfdesk.git
   cd surfdesk
   rustup target add wasm32-unknown-unknown
   cargo install trunk
   ```

2. **Set up development environment**:
   ```bash
   # Install platform-specific dependencies
   # macOS: xcode-select --install
   # Ubuntu: sudo apt install build-essential
   # Windows: Install Visual Studio Build Tools
   ```

3. **Verify multi-platform build**:
   ```bash
   ./scripts/build-all.sh
   ```

### **Development Workflow**
1. **Choose your step**: Start with Step 1.1 in ROADMAP.md
2. **Create feature branch**: `git checkout -b step-[X].[Y]-[feature-name]`
3. **Implement across platforms**: Shared code first, then platform-specific
4. **Test thoroughly**: Verify all platforms work correctly
5. **Update documentation**: Document platform-specific features
6. **Commit and update status**: Mark step as complete

### **Platform-Specific Development**

#### **Desktop Development**
```bash
# Run desktop app with hot reload
cargo run --bin surfdesk-desktop

# Build for release
cargo build --release --bin surfdesk-desktop

# Package for distribution
./scripts/package-desktop.sh
```

#### **Web Development**
```bash
# Serve web app with hot reload
cd surfdesk-web
trunk serve

# Build for production
trunk build --release

# Deploy to static hosting
./scripts/deploy-web.sh
```

#### **Terminal Development**
```bash
# Run terminal interface
cargo run --bin surfdesk-tui

# Test keyboard navigation
cargo test --bin surfdesk-tui -- --nocapture
```

---

## 📞 Support and Troubleshooting

### **Common Multi-Platform Issues**

#### **Build Issues**
- **WASM compilation errors**: Check target installation and dependencies
- **Desktop linking errors**: Verify platform-specific dependencies are installed
- **Terminal rendering issues**: Check terminal compatibility and color support

#### **Runtime Issues**
- **Cross-platform state sync**: Verify serialization compatibility
- **Performance differences**: Profile and optimize per platform
- **Feature inconsistencies**: Check platform adapter implementations

### **Getting Help**
- Review platform-specific documentation in each platform's directory
- Check GitHub issues for known cross-platform problems
- Refer to Dioxus documentation for framework-specific issues
- Create detailed bug reports with platform information

---

## 🎯 Success Metrics

### **Multi-Platform Success**
- **Feature Parity**: 100% feature parity across all platforms
- **Build Success**: All platforms build successfully on CI/CD
- **Test Coverage**: >90% coverage across all components
- **Performance**: Consistent performance within acceptable limits
- **User Experience**: Consistent and intuitive experience across platforms

### **Development Metrics**
- **Code Sharing**: >70% shared code across platforms
- **Build Time**: <5 minutes for full multi-platform build
- **Test Time**: <10 minutes for full test suite execution
- **Documentation**: 100% API coverage with platform examples
- **Onboarding**: <1 day for new developers to set up all platforms

---

**Remember**: You are building a professional, multi-platform development tool. Take pride in your work, ensure cross-platform excellence, maintain high standards throughout implementation, and provide a consistent, high-quality experience across desktop, web, and terminal platforms.

Good luck! 🚀