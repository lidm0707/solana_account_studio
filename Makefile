# =============================================================================
# SurfDesk - Multi-platform Solana Account Studio
# Comprehensive Makefile for development, testing, and deployment
# =============================================================================

# Project Configuration
PROJECT_NAME := surfdesk
VERSION := $(shell grep '^version = ' Cargo.toml | head -1 | cut -d '"' -f 2)
RUST_TOOLCHAIN := $(shell cat rust-toolchain.toml 2>/dev/null | grep '^channel = ' | cut -d '"' -f 2 || echo "stable")

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[1;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
NC := \033[0m # No Color

# Directories
BUILD_DIR := target
DIST_DIR := dist
SCRIPTS_DIR := scripts

# Cargo commands
CARGO := cargo
CARGO_FLAGS := --color always
CARGO_BUILD_FLAGS := $(CARGO_FLAGS) --release
CARGO_TEST_FLAGS := $(CARGO_FLAGS) --verbose

# Platform flags
FEATURES_DESKTOP := desktop
FEATURES_WEB := web
FEATURES_TUI := tui
FEATURES_CLI := cli
FEATURES_ALL := desktop,web,tui,cli

# Default target
.PHONY: default
default: help

# =============================================================================
# Help and Information
# =============================================================================

.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)SurfDesk - Multi-platform Solana Account Studio$(NC)"
	@echo "$(BLUE)Version: $(VERSION) | Rust: $(RUST_TOOLCHAIN)$(NC)"
	@echo ""
	@echo "$(YELLOW)Available targets:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(PURPLE)Examples:$(NC)"
	@echo "  make setup          # Setup development environment"
	@echo "  make build          # Build all platforms"
	@echo "  make run-web        # Run web application"
	@echo "  make test           # Run all tests"
	@echo "  make clean          # Clean build artifacts"

.PHONY: info
info: ## Show project information
	@echo "$(CYAN)Project Information:$(NC)"
	@echo "  Name: $(PROJECT_NAME)"
	@echo "  Version: $(VERSION)"
	@echo "  Rust Toolchain: $(RUST_TOOLCHAIN)"
	@echo "  Build Directory: $(BUILD_DIR)"
	@echo ""
	@echo "$(CYAN)Workspace Members:$(NC)"
	@$(CARGO) metadata --no-deps --format-version 1 | jq -r '.workspace_members[]' | while read member; do \
		if [ "$$member" != "$(PROJECT_NAME)" ]; then \
			echo "  - $$member"; \
		fi; \
	done 2>/dev/null || echo "  Run 'cargo metadata' to see workspace members"

# =============================================================================
# Environment Setup
# =============================================================================

.PHONY: setup
setup: ## Setup development environment
	@echo "$(YELLOW)Setting up development environment...$(NC)"
	@echo "$(BLUE)Checking Rust installation...$(NC)"
	@rustup --version || (echo "$(RED)Rust not installed. Please install Rust first.$(NC)" && exit 1)
	@echo "$(BLUE)Installing required Rust toolchain...$(NC)"
	@rustup install $(RUST_TOOLCHAIN) >/dev/null 2>&1 || echo "$(YELLOW)Toolchain $(RUST_TOOLCHAIN) already installed$(NC)"
	@rustup default $(RUST_TOOLCHAIN)
	@echo "$(BLUE)Installing required components...$(NC)"
	@rustup component add rustfmt clippy rust-src >/dev/null 2>&1 || true
	@echo "$(BLUE)Installing WebAssembly target...$(NC)"
	@rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || echo "$(YELLOW)WASM target already installed$(NC)"
	@echo "$(BLUE)Installing Trunk for web deployment...$(NC)"
	@cargo install trunk --locked >/dev/null 2>&1 || echo "$(YELLOW)Trunk already installed$(NC)"
	@echo "$(GREEN)✓ Development environment setup complete!$(NC)"

.PHONY: setup-dev
setup-dev: setup ## Setup development environment with additional tools
	@echo "$(BLUE)Installing development tools...$(NC)"
	@cargo install cargo-watch cargo-audit cargo-outdated >/dev/null 2>&1 || true
	@echo "$(GREEN)✓ Development tools installed!$(NC)"

.PHONY: sample
sample: ## Create sample configuration files
	@echo "$(YELLOW)Creating sample configuration...$(NC)"
	@mkdir -p config
	@echo "# SurfDesk Configuration" > config/.env.example
	@echo "SOLANA_RPC_URL=https://api.devnet.solana.com" >> config/.env.example
	@echo "LOG_LEVEL=info" >> config/.env.example
	@echo "DATABASE_URL=sqlite://surfdesk.db" >> config/.env.example
	@echo "# Copy this file to .env and update with your settings" >> config/.env.example
	@if [ ! -f config/.env ]; then \
		cp config/.env.example config/.env; \
		echo "$(GREEN)✓ Created config/.env from example$(NC)"; \
	else \
		echo "$(YELLOW)config/.env already exists$(NC)"; \
	fi
	@echo "$(GREEN)✓ Sample configuration created!$(NC)"

# =============================================================================
# Building
# =============================================================================

.PHONY: build
build: ## Build all platforms
	@echo "$(YELLOW)Building all platforms...$(NC)"
	@$(CARGO) build $(CARGO_BUILD_FLAGS) --workspace --all-targets
	@echo "$(GREEN)✓ All platforms built successfully!$(NC)"

.PHONY: build-desktop
build-desktop: ## Build desktop application
	@echo "$(YELLOW)Building desktop application...$(NC)"
	@$(CARGO) build $(CARGO_BUILD_FLAGS) --bin surfdesk-desktop --features $(FEATURES_DESKTOP)
	@echo "$(GREEN)✓ Desktop application built!$(NC)"

.PHONY: build-web
build-web: ## Build web application
	@echo "$(YELLOW)Building web application...$(NC)"
	@cd surfdesk-web && trunk build $(CARGO_BUILD_FLAGS)
	@echo "$(GREEN)✓ Web application built!$(NC)"

.PHONY: build-tui
build-tui: ## Build terminal UI application
	@echo "$(YELLOW)Building terminal UI application...$(NC)"
	@$(CARGO) build $(CARGO_BUILD_FLAGS) --bin surfdesk-tui --features $(FEATURES_TUI)
	@echo "$(GREEN)✓ Terminal UI application built!$(NC)"

.PHONY: build-cli
build-cli: ## Build CLI application
	@echo "$(YELLOW)Building CLI application...$(NC)"
	@$(CARGO) build $(CARGO_BUILD_FLAGS) --bin surfdesk-cli --features $(FEATURES_CLI)
	@echo "$(GREEN)✓ CLI application built!$(NC)"

# =============================================================================
# Development and Running
# =============================================================================

.PHONY: dev
dev: ## Start development with hot reload (web)
	@echo "$(YELLOW)Starting web development server...$(NC)"
	@cd surfdesk-web && trunk serve

.PHONY: run-desktop
run-desktop: build-desktop ## Run desktop application
	@echo "$(YELLOW)Running desktop application...$(NC)"
	@$(BUILD_DIR)/release/surfdesk-desktop

.PHONY: run-web
run-web: build-web ## Run web application
	@echo "$(YELLOW)Serving web application...$(NC)"
	@cd surfdesk-web && python3 -m http.server 8080 --directory dist 2>/dev/null || \
		cd surfdesk-web && python -m http.server 8080 --directory dist 2>/dev/null || \
		echo "$(RED)Please start a web server manually in surfdesk-web/dist$(NC)"

.PHONY: run-tui
run-tui: build-tui ## Run terminal UI application
	@echo "$(YELLOW)Running terminal UI application...$(NC)"
	@$(BUILD_DIR)/release/surfdesk-tui

.PHONY: run-cli
run-cli: build-cli ## Run CLI application
	@echo "$(YELLOW)Running CLI application...$(NC)"
	@$(BUILD_DIR)/release/surfdesk-cli --help

.PHONY: watch
watch: ## Watch for changes and rebuild
	@echo "$(YELLOW)Watching for changes...$(NC)"
	@cargo watch -x 'build --release --workspace'

.PHONY: watch-desktop
watch-desktop: ## Watch and run desktop
	@echo "$(YELLOW)Watching and running desktop...$(NC)"
	@cargo watch -x 'run --bin surfdesk-desktop'

.PHONY: watch-web
watch-web: ## Watch and serve web
	@echo "$(YELLOW)Watching and serving web...$(NC)"
	@cd surfdesk-web && trunk serve

# =============================================================================
# Testing
# =============================================================================

.PHONY: test
test: ## Run all tests
	@echo "$(YELLOW)Running all tests...$(NC)"
	@$(CARGO) test $(CARGO_TEST_FLAGS) --workspace
	@echo "$(GREEN)✓ All tests passed!$(NC)"

.PHONY: test-core
test-core: ## Run core library tests
	@echo "$(YELLOW)Running core library tests...$(NC)"
	@$(CARGO) test $(CARGO_TEST_FLAGS) --package surfdesk-core

.PHONY: test-unit
test-unit: ## Run unit tests only
	@echo "$(YELLOW)Running unit tests...$(NC)"
	@$(CARGO) test $(CARGO_TEST_FLAGS) --lib

.PHONY: test-integration
test-integration: ## Run integration tests
	@echo "$(YELLOW)Running integration tests...$(NC)"
	@$(CARGO) test $(CARGO_TEST_FLAGS) --test '*'

.PHONY: test-coverage
test-coverage: ## Run tests with coverage
	@echo "$(YELLOW)Running tests with coverage...$(NC)"
	@cargo install cargo-tarpaulin >/dev/null 2>&1 || true
	@cargo tarpaulin --workspace --output-dir $(BUILD_DIR)/coverage --out Html
	@echo "$(GREEN)✓ Coverage report generated in $(BUILD_DIR)/coverage/tarpaulin-report.html$(NC)"

# =============================================================================
# Code Quality
# =============================================================================

.PHONY: check
check: ## Run all code quality checks
	@echo "$(YELLOW)Running code quality checks...$(NC)"
	@$(MAKE) fmt
	@$(MAKE) clippy
	@$(MAKE) audit
	@echo "$(GREEN)✓ All code quality checks passed!$(NC)"

.PHONY: fmt
fmt: ## Format code with rustfmt
	@echo "$(YELLOW)Formatting code...$(NC)"
	@$(CARGO) fmt --all
	@echo "$(GREEN)✓ Code formatted!$(NC)"

.PHONY: fmt-check
fmt-check: ## Check if code is formatted
	@echo "$(YELLOW)Checking code formatting...$(NC)"
	@$(CARGO) fmt --all -- --check
	@echo "$(GREEN)✓ Code is properly formatted!$(NC)"

.PHONY: clippy
clippy: ## Run clippy lints
	@echo "$(YELLOW)Running clippy...$(NC)"
	@$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✓ Clippy checks passed!$(NC)"

.PHONY: audit
audit: ## Audit dependencies for security vulnerabilities
	@echo "$(YELLOW)Auditing dependencies...$(NC)"
	@$(CARGO) audit
	@echo "$(GREEN)✓ No security vulnerabilities found!$(NC)"

.PHONY: outdated
outdated: ## Check for outdated dependencies
	@echo "$(YELLOW)Checking for outdated dependencies...$(NC)"
	@cargo outdated --workspace

# =============================================================================
# Database and Migration
# =============================================================================

.PHONY: db-setup
db-setup: ## Setup database
	@echo "$(YELLOW)Setting up database...$(NC)"
	@if [ -f config/.env ]; then \
		$(CARGO) run --bin surfdesk-cli -- db setup; \
		echo "$(GREEN)✓ Database setup complete!$(NC)"; \
	else \
		echo "$(RED)config/.env not found. Run 'make sample' first.$(NC)"; \
		exit 1; \
	fi

.PHONY: db-migrate
db-migrate: ## Run database migrations
	@echo "$(YELLOW)Running database migrations...$(NC)"
	@$(CARGO) run --bin surfdesk-cli -- db migrate
	@echo "$(GREEN)✓ Database migrations complete!$(NC)"

.PHONY: db-reset
db-reset: ## Reset database
	@echo "$(YELLOW)Resetting database...$(NC)"
	@$(CARGO) run --bin surfdesk-cli -- db reset
	@echo "$(GREEN)✓ Database reset complete!$(NC)"

# =============================================================================
# Cleaning
# =============================================================================

.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	@$(CARGO) clean
	@cd surfdesk-web && trunk clean 2>/dev/null || true
	@echo "$(GREEN)✓ Build artifacts cleaned!$(NC)"

.PHONY: clean-all
clean-all: clean ## Clean all generated files
	@echo "$(YELLOW)Cleaning all generated files...$(NC)"
	@$(CARGO) clean
	@rm -rf $(DIST_DIR)
	@cd surfdesk-web && trunk clean 2>/dev/null || true
	@rm -f config/.env
	@echo "$(GREEN)✓ All generated files cleaned!$(NC)"

.PHONY: clean-cache
clean-cache: ## Clean cargo cache
	@echo "$(YELLOW)Cleaning cargo cache...$(NC)"
	@$(CARGO) clean --release
	@echo "$(GREEN)✓ Cargo cache cleaned!$(NC)"

# =============================================================================
# Utilities
# =============================================================================

.PHONY: tree
tree: ## Show project structure
	@echo "$(CYAN)Project Structure:$(NC)"
	@tree -I 'target|node_modules|.git|dist' -a --dirsfirst

.PHONY: deps
deps: ## Show dependency tree
	@echo "$(CYAN)Dependency Tree:$(NC)"
	@$(CARGO) tree --workspace --format "{p}"

.PHONY: size
size: ## Show binary sizes
	@echo "$(CYAN)Binary Sizes:$(NC)"
	@if [ -d $(BUILD_DIR)/release ]; then \
		for binary in $(BUILD_DIR)/release/surfdesk-*; do \
			if [ -f "$$binary" ]; then \
				size=$$(ls -lh "$$binary" | awk '{print $$5}'); \
				echo "  $$(basename $$binary): $$size"; \
			fi; \
		done; \
	else \
		echo "$(YELLOW)No release binaries found. Run 'make build' first.$(NC)"; \
	fi

.PHONY: docs
docs: ## Generate documentation
	@echo "$(YELLOW)Generating documentation...$(NC)"
	@$(CARGO) doc --workspace --no-deps --open
	@echo "$(GREEN)✓ Documentation generated!$(NC)"

.PHONY: version
version: ## Show version information
	@echo "$(CYAN)Version Information:$(NC)"
	@echo "  Project: $(PROJECT_NAME)"
	@echo "  Version: $(VERSION)"
	@echo "  Rust: $(RUST_TOOLCHAIN)"
	@echo "  Cargo: $(shell cargo --version | cut -d' ' -f2)"
	@echo "  Trunk: $(shell trunk --version 2>/dev/null | cut -d' ' -f2 || echo 'Not installed')"

# =============================================================================
# Release and Distribution
# =============================================================================

.PHONY: release
release: check test build ## Create release build
	@echo "$(YELLOW)Creating release...$(NC)"
	@mkdir -p $(DIST_DIR)
	@echo "$(VERSION)" > $(DIST_DIR)/VERSION
	@echo "$(GREEN)✓ Release created in $(DIST_DIR)!$(NC)"

.PHONY: package
package: release ## Package for distribution
	@echo "$(YELLOW)Packaging for distribution...$(NC)"
	@mkdir -p $(DIST_DIR)/packages
	@tar -czf $(DIST_DIR)/packages/$(PROJECT_NAME)-$(VERSION)-desktop.tar.gz -C $(BUILD_DIR)/release surfdesk-desktop
	@tar -czf $(DIST_DIR)/packages/$(PROJECT_NAME)-$(VERSION)-tui.tar.gz -C $(BUILD_DIR)/release surfdesk-tui
	@tar -czf $(DIST_DIR)/packages/$(PROJECT_NAME)-$(VERSION)-cli.tar.gz -C $(BUILD_DIR)/release surfdesk-cli
	@tar -czf $(DIST_DIR)/packages/$(PROJECT_NAME)-$(VERSION)-web.tar.gz -C surfdesk-web/dist .
	@echo "$(GREEN)✓ Packages created in $(DIST_DIR)/packages!$(NC)"

# =============================================================================
# Quick Start Targets
# =============================================================================

.PHONY: quickstart
quickstart: setup sample build-web ## Quick start for new developers
	@echo "$(GREEN)🚀 Quick start complete!$(NC)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(NC)"
	@echo "  1. Review config/.env and update settings"
	@echo "  2. Run 'make run-web' to start the web application"
	@echo "  3. Open http://localhost:8080 in your browser"
	@echo "  4. Run 'make run-desktop' for the desktop application"
	@echo ""
	@echo "$(BLUE)For more options, run 'make help'$(NC)"

.PHONY: dev-setup
dev-setup: setup-dev sample ## Complete development setup
	@echo "$(GREEN)🛠️  Development setup complete!$(NC)"
	@echo ""
	@echo "$(YELLOW)Development commands:$(NC)"
	@echo "  make dev          # Start web development server"
	@echo "  make watch        # Watch for changes"
	@echo "  make test         # Run tests"
	@echo "  make check        # Run code quality checks"
	@echo ""
	@echo "$(BLUE)Happy coding! 🎯$(NC)"

# =============================================================================
# Advanced Targets
# =============================================================================

.PHONY: benchmark
benchmark: ## Run benchmarks
	@echo "$(YELLOW)Running benchmarks...$(NC)"
	@$(CARGO) bench --workspace

.PHONY: profile
profile: ## Profile the application
	@echo "$(YELLOW)Profiling application...$(NC)"
	@$(CARGO) build --release --features profiling
	@echo "$(GREEN)✓ Profile build complete!$(NC)"

.PHONY: flamegraph
flamegraph: ## Generate flamegraph
	@echo "$(YELLOW)Generating flamegraph...$(NC)"
	@cargo install flamegraph >/dev/null 2>&1 || true
	@cargo flamegraph --bin surfdesk-desktop
	@echo "$(GREEN)✓ Flamegraph generated!$(NC)"

.PHONY: install-tools
install-tools: ## Install all development tools
	@echo "$(YELLOW)Installing all development tools...$(NC)"
	@cargo install cargo-watch cargo-audit cargo-outdated cargo-tarpaulin cargo-flamegraph trunk
	@echo "$(GREEN)✓ All development tools installed!$(NC)"

# =============================================================================
# Special Targets
# =============================================================================

# Prevent make from treating files with these names as targets
.PHONY: all
all: build test check

# Always run these targets
.PHONY: always
always:

# Include custom makefile if it exists
-include Makefile.local

# =============================================================================
# Help Summary (End of File)
# =============================================================================

# Quick reference for common tasks:
# make setup       - Setup development environment
# make sample      - Create sample config files
# make dev         - Start development server
# make build       - Build all platforms
# make test        - Run tests
# make clean       - Clean build artifacts
# make help        - Show this help message
