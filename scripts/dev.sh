#!/bin/bash

# SurfDesk Development Script
# Provides hot reload and development environment setup

set -e

echo "🚀 SurfDesk Development Environment"
echo "=================================="

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install dependencies if missing
install_dependencies() {
    echo "📦 Checking dependencies..."

    if ! command_exists cargo; then
        echo "❌ Rust not found. Please install Rust from https://rustup.rs/"
        exit 1
    fi

    if ! command_exists trunk; then
        echo "📦 Installing trunk for web development..."
        cargo install trunk
    fi

    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo "📦 Installing wasm32 target..."
        rustup target add wasm32-unknown-unknown
    fi

    echo "✅ Dependencies ready"
}

# Function to setup development environment
setup_dev() {
    echo "🔧 Setting up development environment..."

    # Create .env file if it doesn't exist
    if [ ! -f .env ]; then
        cat > .env << EOF
# SurfDesk Development Environment
RUST_LOG=debug
RUST_BACKTRACE=1
SURFDESK_RPC_URL=http://localhost:8899
SURFDESK_ENV=development
EOF
        echo "📝 Created .env file"
    fi

    # Install pre-commit hooks
    if [ -d .git ]; then
        echo "🔧 Setting up pre-commit hooks..."
        cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
echo "🔍 Running pre-commit checks..."

# Format code
echo "📝 Formatting code..."
cargo fmt --all

# Run clippy
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
echo "🧪 Running tests..."
cargo test --all-features --workspace

echo "✅ Pre-commit checks passed"
EOF
        chmod +x .git/hooks/pre-commit
        echo "✅ Pre-commit hooks installed"
    fi
}

# Function to start desktop development
run_desktop() {
    echo "🖥️  Starting desktop development server..."
    echo "🔄 Hot reload enabled"
    echo "📱 Open http://localhost:8080 to view the app"
    echo ""

    cd surfdesk-desktop
    RUST_LOG=debug cargo run
}

# Function to start web development
run_web() {
    echo "🌐 Starting web development server..."
    echo "🔄 Hot reload enabled"
    echo "📱 Open http://localhost:8080 to view the app"
    echo ""

    cd surfdesk-web
    RUST_LOG=debug trunk serve --open --port 8080
}

# Function to start terminal development
run_tui() {
    echo "💻 Starting terminal interface..."
    echo "🔄 Hot reload enabled"
    echo ""

    cd surfdesk-tui
    RUST_LOG=debug cargo run
}

# Function to run all tests
run_tests() {
    echo "🧪 Running test suite..."
    cargo test --all-features --workspace -- --nocapture
}

# Function to build all targets
build_all() {
    echo "🏗️  Building all targets..."

    echo "📦 Building desktop..."
    cargo build --release --bin surfdesk-desktop

    echo "🌐 Building web..."
    cd surfdesk-web && trunk build --release && cd ..

    echo "💻 Building terminal..."
    cargo build --release --bin surfdesk-tui

    echo "🛠️  Building CLI..."
    cargo build --release --bin surfdesk-cli

    echo "✅ All targets built successfully"
}

# Function to clean everything
clean_all() {
    echo "🧹 Cleaning build artifacts..."
    cargo clean
    rm -rf surfdesk-web/dist
    echo "✅ Clean completed"
}

# Function to show help
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  setup     - Setup development environment"
    echo "  desktop   - Start desktop development server"
    echo "  web       - Start web development server"
    echo "  tui       - Start terminal interface"
    echo "  test      - Run test suite"
    echo "  build     - Build all targets"
    echo "  clean     - Clean build artifacts"
    echo "  help      - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 setup      # Initial setup"
    echo "  $0 desktop    # Start desktop development"
    echo "  $0 web        # Start web development"
    echo "  $0 test       # Run tests"
}

# Main script logic
case "${1:-help}" in
    setup)
        install_dependencies
        setup_dev
        ;;
    desktop)
        install_dependencies
        run_desktop
        ;;
    web)
        install_dependencies
        run_web
        ;;
    tui)
        install_dependencies
        run_tui
        ;;
    test)
        run_tests
        ;;
    build)
        build_all
        ;;
    clean)
        clean_all
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "❌ Unknown command: $1"
        show_help
        exit 1
        ;;
esac
