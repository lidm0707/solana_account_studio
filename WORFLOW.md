# Multi-Platform Development Workflow for SurfDesk (Dioxus 0.6+)

## Overview

This document defines the comprehensive development workflow for SurfDesk, a multi-platform Solana account studio built with Dioxus 0.6+. The workflow supports simultaneous development across desktop, web, and terminal platforms, ensuring consistency and efficiency throughout the development lifecycle.

## 🏗️ Architecture-Driven Development

### Multi-Platform Workspace Structure

```
solana_account_studio/
├── surfdesk-core/           # Shared business logic
│   ├── src/
│   │   ├── services/        # Platform-agnostic services
│   │   ├── components/      # Cross-platform UI components
│   │   ├── platform/        # Platform abstractions
│   │   └── utils/           # Shared utilities
│   └── Cargo.toml
├── surfdesk-desktop/       # Desktop application
├── surfdesk-web/           # Web application
├── surfdesk-tui/           # Terminal interface
├── surfdesk-cli/           # Headless CLI
└── scripts/                # Build and automation scripts
```

### Development Principles

1. **Shared First**: Implement business logic in `surfdesk-core` first
2. **Platform Adaptation**: Create platform-specific implementations only when necessary
3. **Cross-Platform Testing**: Test features across all platforms simultaneously
4. **Consistent API**: Maintain consistent interfaces across platforms
5. **Progressive Enhancement**: Start with shared features, add platform-specific capabilities

## 🔄 Development Lifecycle

### Phase 1: Foundation Development

#### 1.1 Core Service Implementation
```bash
# Start with shared business logic
cd surfdesk-core
cargo add dioxus dioxus-router dioxus-signals
cargo add solana-sdk solana-client tokio serde serde_json

# Implement core services
mkdir -p src/services/{solana,surfpool,database,config}
mkdir -p src/components/{ui,layout,forms}
mkdir -p src/platform/{desktop,web,terminal}
```

#### 1.2 Platform Abstraction Layer
```rust
// surfdesk-core/src/platform/mod.rs
pub trait PlatformAdapter {
    fn show_file_dialog(&self, options: FileDialogOptions) -> Result<String, PlatformError>;
    fn show_notification(&self, message: &str) -> Result<(), PlatformError>;
    fn get_storage_path(&self) -> PathBuf;
    fn get_system_info(&self) -> SystemInfo;
    fn platform_type(&self) -> PlatformType;
}

pub enum PlatformType {
    Desktop,
    Web,
    Terminal,
}
```

#### 1.3 Shared Component Development
```rust
// surfdesk-core/src/components/button.rs
#[component]
pub fn Button(
    children: Element,
    onclick: EventHandler<MouseEvent>,
    variant: ButtonVariant,
    platform: PlatformType,
) -> Element {
    rsx! {
        button {
            class: format!("btn btn-{variant} platform-{platform:?}"),
            onclick: onclick,
            {children}
        }
    }
}
```

### Phase 2: Platform-Specific Implementation

#### 2.1 Desktop Platform Development
```bash
cd surfdesk-desktop
cargo add dioxus --features desktop
cargo add surfdesk-core --path ../surfdesk-core
cargo add tauri-plugin-window-state
```

```rust
// surfdesk-desktop/src/main.rs
use dioxus_desktop::*;
use surfdesk_core::*;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_window(dioxus_desktop::WindowBuilder::new()
                .with_title("SurfDesk")
                .with_inner_size(dioxus_desktop::tao::dpi::LogicalSize::new(1200, 800))
            )
    );
}

#[component]
fn App() -> Element {
    rsx! {
        DesktopLayout {
            sidebar: Sidebar {},
            main_content: MainContent {},
            menu_bar: MenuBar {},
        }
    }
}
```

#### 2.2 Web Platform Development
```bash
cd surfdesk-web
cargo add dioxus --features web
cargo add surfdesk-core --path ../surfdesk-core
cargo add gloo-web console_log log
```

```rust
// surfdesk-web/src/main.rs
use dioxus_web::*;
use surfdesk_core::*;

fn main() {
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize log");
    
    dioxus_web::launch_cfg(
        App,
        dioxus_web::Config::new().pre_root("root")
    );
}

#[component]
fn App() -> Element {
    rsx! {
        WebLayout {
            navigation: Navigation {},
            main_content: MainContent {},
            footer: Footer {},
        }
    }
}
```

#### 2.3 Terminal Platform Development
```bash
cd surfdesk-tui
cargo add crossterm ratatui
cargo add surfdesk-core --path ../surfdesk-core
```

```rust
// surfdesk-tui/src/main.rs
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use surfdesk_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = SurfDeskTui::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
```

## 🛠️ Development Tools & Environment

### Required Toolchain

```bash
# Install Rust with required targets
rustup update stable
rustup target add wasm32-unknown-unknown
rustup component add rustfmt clippy

# Install Dioxus CLI
cargo install dioxus-cli

# Install Trunk for web builds
cargo install trunk

# Install development dependencies
cargo install cargo-watch cargo-edit
```

### Development Scripts

```bash
#!/bin/bash
# scripts/dev.sh - Development server script

echo "🚀 Starting SurfDesk multi-platform development..."

# Start desktop app in background
echo "🖥️  Starting desktop app..."
cd surfdesk-desktop && cargo run &

# Start web development server
echo "🌐 Starting web dev server..."
cd surfdesk-web && trunk serve &

# Start terminal interface (optional)
echo "💻 Terminal interface available with: cargo run --bin surfdesk-tui"

echo "✅ Development environment started!"
echo "📊 Desktop: http://localhost:3000"
echo "🌐 Web: http://localhost:8080"
```

```bash
#!/bin/bash
# scripts/build.sh - Build all platforms

echo "🏗️  Building SurfDesk for all platforms..."

# Build desktop
echo "🖥️  Building desktop..."
cd surfdesk-desktop && cargo build --release

# Build web
echo "🌐 Building web..."
cd surfdesk-web && trunk build --release

# Build terminal
echo "💻 Building terminal..."
cd surfdesk-tui && cargo build --release

# Build CLI
echo "⚡ Building CLI..."
cd surfdesk-cli && cargo build --release

echo "✅ All platforms built successfully!"
```

## 🧪 Testing Strategy

### Multi-Platform Testing Workflow

```bash
#!/bin/bash
# scripts/test.sh - Run tests for all platforms

echo "🧪 Running SurfDesk test suite..."

# Test shared core
echo "🔧 Testing core library..."
cd surfdesk-core && cargo test --lib

# Test desktop functionality
echo "🖥️  Testing desktop..."
cd surfdesk-desktop && cargo test

# Test web functionality
echo "🌐 Testing web..."
cd surfdesk-web && cargo test

# Test terminal functionality
echo "💻 Testing terminal..."
cd surfdesk-tui && cargo test

# Run integration tests
echo "🔗 Running integration tests..."
cargo test --test integration

echo "✅ All tests completed!"
```

### Cross-Platform Integration Tests

```rust
// tests/integration_test.rs
use surfdesk_core::*;
use surfdesk_desktop::*;
use surfdesk_web::*;
use surfdesk_tui::*;

#[tokio::test]
async fn test_cross_platform_consistency() {
    // Test that all platforms produce consistent results
    let desktop_app = SurfDeskDesktop::new().await.unwrap();
    let web_app = SurfDeskWeb::new().await.unwrap();
    let terminal_app = SurfDeskTui::new().await.unwrap();
    
    // Create test project on each platform
    let desktop_project = desktop_app.create_project("Test Project").await.unwrap();
    let web_project = web_app.create_project("Test Project").await.unwrap();
    let terminal_project = terminal_app.create_project("Test Project").await.unwrap();
    
    // Verify consistency
    assert_eq!(desktop_project.name, web_project.name);
    assert_eq!(web_project.name, terminal_project.name);
}
```

### End-to-End Testing

```rust
// tests/e2e_test.rs
#[tokio::test]
async fn test_complete_workflow() {
    let mut app = SurfDeskApp::new(Platform::Desktop).await.unwrap();
    
    // Complete workflow test
    let project = app.create_project("E2E Test Project").await.unwrap();
    let environment = app.create_environment(&project.id, EnvironmentType::Local).await.unwrap();
    
    app.start_environment(&environment.id).await.unwrap();
    
    let program = app.deploy_program(&project.id, "./test_program.so").await.unwrap();
    let transaction = app.create_transaction(&program.id).await.unwrap();
    
    let signature = app.send_transaction(transaction).await.unwrap();
    assert!(!signature.is_empty());
    
    app.stop_environment(&environment.id).await.unwrap();
}
```

## 🚀 Deployment & Release

### Multi-Platform Release Pipeline

```yaml
# .github/workflows/release.yml
name: Release Multi-Platform

on:
  push:
    tags: ['v*']

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      - name: Run tests
        run: ./scripts/test.sh

  build-desktop:
    needs: test
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - name: Build desktop
        run: |
          cd surfdesk-desktop
          cargo build --release
      - name: Upload artifacts
        uses: actions/upload-artifact@v3

  build-web:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build web
        run: |
          cd surfdesk-web
          trunk build --release
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./surfdesk-web/dist

  release:
    needs: [build-desktop, build-web]
    runs-on: ubuntu-latest
    steps:
      - name: Create Release
        uses: actions/create-release@v1
        with:
          tag_name: ${{ github.ref }}
          release_name: SurfDesk ${{ github.ref }}
```

### Platform-Specific Deployment

#### Desktop Deployment
```bash
# scripts/package-desktop.sh
#!/bin/bash

echo "📦 Packaging SurfDesk desktop application..."

# Create package directory
mkdir -p dist/desktop

# Build for each platform
echo "🪟 Building for Windows..."
cargo build --target x86_64-pc-windows-gnu --release

echo "🍎 Building for macOS..."
cargo build --target x86_64-apple-darwin --release

echo "🐧 Building for Linux..."
cargo build --target x86_64-unknown-linux-gnu --release

# Create installers
echo "📦 Creating installers..."
# Platform-specific packaging logic

echo "✅ Desktop packaging complete!"
```

#### Web Deployment
```bash
# scripts/deploy-web.sh
#!/bin/bash

echo "🌐 Deploying SurfDesk web application..."

# Build for production
cd surfdesk-web
trunk build --release

# Deploy to hosting
if [ "$DEPLOY_TARGET" = "netlify" ]; then
    netlify deploy --prod --dir=dist
elif [ "$DEPLOY_TARGET" = "vercel" ]; then
    vercel --prod
fi

echo "✅ Web deployment complete!"
```

## 🔄 Continuous Integration Workflow

### Pre-Commit Hooks

```bash
#!/bin/sh
# .git/hooks/pre-commit

echo "🔍 Running pre-commit checks..."

# Format code
echo "📝 Formatting code..."
cargo fmt --all

# Run clippy
echo "⚡ Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
echo "🧪 Running tests..."
cargo test --lib

# Check for security vulnerabilities
echo "🔒 Checking for security issues..."
cargo audit

echo "✅ Pre-commit checks passed!"
```

### Development Workflow Commands

```bash
# Development commands
make dev          # Start development servers
make build        # Build all platforms
make test         # Run all tests
make lint         # Run linting
make format       # Format code
make clean        # Clean build artifacts
make release      # Build release versions

# Platform-specific commands
make dev-desktop  # Start desktop dev server
make dev-web      # Start web dev server
make dev-tui      # Start terminal interface

# Testing commands
make test-unit    # Run unit tests
make test-integration # Run integration tests
make test-e2e     # Run end-to-end tests
make test-coverage # Generate coverage report
```

## 📋 Code Review Process

### Review Checklist

#### Core Components
- [ ] Follows Dioxus 0.6+ best practices
- [ ] Platform-agnostic implementation is correct
- [ ] Error handling is comprehensive
- [ ] Documentation is complete
- [ ] Tests are thorough

#### Platform-Specific Components
- [ ] Platform integration is correct
- [ ] Native features are properly used
- [ ] Performance is optimized for the platform
- [ ] User experience is consistent
- [ ] Platform-specific tests pass

#### Cross-Platform Consistency
- [ ] Feature parity across platforms
- [ ] Consistent API design
- [ ] Shared state management works correctly
- [ ] UI/UX consistency
- [ ] Performance benchmarks met

### Review Process

1. **Self-Review**: Author reviews their own code against checklist
2. **Peer Review**: Another developer reviews the code
3. **Platform Review**: Platform-specific experts review relevant parts
4. **Integration Review**: Cross-platform integration is tested
5. **Approval**: Code is approved and merged

## 🐛 Debugging & Troubleshooting

### Multi-Platform Debugging

#### Desktop Debugging
```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin surfdesk-desktop

# Use debugger
rust-gdb target/debug/surfdesk-desktop
```

#### Web Debugging
```bash
# Enable web console logging
console_log::init_with_level(log::Level::Debug);

# Use browser developer tools
# Network tab for WebSocket connections
# Console for WASM errors
# Performance for profiling
```

#### Terminal Debugging
```bash
# Enable verbose logging
RUST_LOG=debug cargo run --bin surfdesk-tui

# Use debug mode
cargo run --bin surfdesk-tui -- --debug
```

### Common Issues & Solutions

#### WASM Compilation Issues
```bash
# Check target is installed
rustup target list --installed | grep wasm32

# Install missing target
rustup target add wasm32-unknown-unknown

# Clear trunk cache
trunk clean
```

#### Platform-Specific Issues
```bash
# Desktop: Check system dependencies
sudo apt-get install libssl-dev pkg-config  # Ubuntu
brew install openssl pkg-config             # macOS

# Web: Check browser compatibility
# Ensure modern browser with WASM support

# Terminal: Check terminal compatibility
# Ensure terminal supports ANSI colors and UTF-8
```

## 📊 Performance Monitoring

### Cross-Platform Metrics

```rust
// surfdesk-core/src/monitoring.rs
pub struct PerformanceMonitor {
    platform: PlatformType,
    metrics: PlatformMetrics,
}

impl PerformanceMonitor {
    pub fn collect_metrics(&self) -> PlatformMetrics {
        match self.platform {
            PlatformType::Desktop => self.collect_desktop_metrics(),
            PlatformType::Web => self.collect_web_metrics(),
            PlatformType::Terminal => self.collect_terminal_metrics(),
        }
    }
    
    fn collect_desktop_metrics(&self) -> PlatformMetrics {
        PlatformMetrics {
            memory_usage: get_process_memory(),
            cpu_usage: get_process_cpu(),
            startup_time: get_startup_time(),
        }
    }
    
    fn collect_web_metrics(&self) -> PlatformMetrics {
        PlatformMetrics {
            bundle_size: get_wasm_bundle_size(),
            load_time: get_page_load_time(),
            fps: get_render_fps(),
        }
    }
    
    fn collect_terminal_metrics(&self) -> PlatformMetrics {
        PlatformMetrics {
            render_time: get_terminal_render_time(),
            input_latency: get_input_latency(),
            memory_usage: get_process_memory(),
        }
    }
}
```

## 🔮 Future Workflow Enhancements

### Planned Improvements

1. **Automated Cross-Platform Testing**: CI pipeline that tests all platforms automatically
2. **Performance Benchmarking**: Automated performance regression testing
3. **Code Generation**: Generate platform-specific boilerplate code
4. **Hot Module Replacement**: Live reloading across all platforms
5. **Visual Testing**: Automated UI testing across platforms
6. **Documentation Generation**: Auto-generate platform-specific documentation

### Tool Development

1. **Custom CLI Tool**: Enhanced `surfdesk-cli` for workflow automation
2. **IDE Integration**: VS Code extension for multi-platform development
3. **Debug Tools**: Platform-specific debugging utilities
4. **Performance Profiler**: Cross-platform performance analysis tool

---

**This workflow provides a comprehensive foundation for developing SurfDesk across multiple platforms, ensuring consistency, quality, and efficiency throughout the development lifecycle.**

**Version**: 1.0  
**Last Updated**: 2025-06-18  
**Framework**: Dioxus 0.6+  
**Platforms**: Desktop, Web, Terminal