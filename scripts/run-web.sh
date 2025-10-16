#!/bin/bash
# 🌐 SurfDesk Web Application Runner
# This script builds and serves the SurfDesk web application

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}=================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}=================================${NC}"
}

# Check if we're in the right directory
check_directory() {
    if [ ! -f "Cargo.toml" ] || [ ! -d "surfdesk-web" ]; then
        print_error "Not in SurfDesk project directory!"
        print_error "Please run this script from the solana_account_studio root directory"
        exit 1
    fi
}

# Check dependencies
check_dependencies() {
    print_status "Checking dependencies..."

    # Check for wasm-pack (recommended for web builds)
    if ! command -v wasm-pack &> /dev/null; then
        print_warning "wasm-pack not found. Installing..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi

    # Check for basic web server (python is usually available)
    if command -v python3 &> /dev/null; then
        WEB_SERVER="python3 -m http.server"
    elif command -v python &> /dev/null; then
        WEB_SERVER="python -m http.server"
    elif command -v node &> /dev/null; then
        if command -v npx &> /dev/null; then
            WEB_SERVER="npx serve"
        else
            print_error "No web server found. Please install Python or Node.js"
            exit 1
        fi
    else
        print_error "No web server found. Please install Python or Node.js"
        exit 1
    fi

    print_status "Using web server: $WEB_SERVER"
}

# Build the web application
build_web() {
    print_header "🏗️ Building SurfDesk Web Application"

    # Navigate to web directory
    cd surfdesk-web

    # Build for web target
    print_status "Building web application..."
    if wasm-pack build --target web --out-dir pkg --dev; then
        print_status "✅ Web application built successfully"
    else
        print_warning "wasm-pack build failed, trying cargo build..."
        if cargo build --release --target wasm32-unknown-unknown; then
            print_status "✅ Cargo build successful"
        else
            print_error "❌ Build failed"
            exit 1
        fi
    fi

    # Go back to root directory
    cd ..

    # Create dist directory
    mkdir -p dist

    # Copy necessary files to dist
    print_status "Setting up distribution directory..."

    # Copy the built WASM files
    if [ -d "surfdesk-web/pkg" ]; then
        cp -r surfdesk-web/pkg/* dist/
    fi

    # Create index.html if it doesn't exist
    if [ ! -f "dist/index.html" ]; then
        print_status "Creating index.html..."
        cat > dist/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SurfDesk - Solana Account Studio</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            text-align: center;
            color: white;
            max-width: 600px;
            padding: 2rem;
        }
        .logo {
            font-size: 3rem;
            margin-bottom: 1rem;
        }
        h1 {
            margin: 0;
            font-size: 2.5rem;
            margin-bottom: 1rem;
        }
        .loading {
            font-size: 1.2rem;
            margin-bottom: 2rem;
            opacity: 0.8;
        }
        .error {
            background: rgba(255, 255, 255, 0.1);
            padding: 1rem;
            border-radius: 8px;
            margin: 1rem 0;
        }
        .features {
            margin-top: 2rem;
            text-align: left;
            background: rgba(255, 255, 255, 0.1);
            padding: 1.5rem;
            border-radius: 8px;
        }
        .features h3 {
            margin-top: 0;
        }
        .features ul {
            margin: 0;
            padding-left: 1.5rem;
        }
        .features li {
            margin: 0.5rem 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="logo">🏄‍♂️</div>
        <h1>SurfDesk</h1>
        <div class="loading" id="loading">Loading your Solana Account Studio...</div>
        <div class="error" id="error" style="display: none;">
            <h3>Application Loading Error</h3>
            <p id="error-message">The web application failed to load.</p>
        </div>

        <div class="features">
            <h3>🌟 Features</h3>
            <ul>
                <li>✅ Multi-Account Management</li>
                <li>✅ Transaction Builder</li>
                <li>✅ Real-time Balance Monitoring</li>
                <li>✅ Network Switching</li>
                <li>✅ Cross-Platform Support</li>
            </ul>
        </div>
    </div>

    <script type="module">
        // Import the main application
        import init, { main } from './surfdesk_web.js';

        async function run() {
            try {
                await init();
                await main();
                document.getElementById('loading').style.display = 'none';
            } catch (error) {
                console.error('Failed to load application:', error);
                document.getElementById('loading').style.display = 'none';
                document.getElementById('error').style.display = 'block';
                document.getElementById('error-message').textContent = error.message;
            }
        }

        run();
    </script>
</body>
</html>
EOF
    fi
}

# Serve the web application
serve_web() {
    print_header "🌐 Serving SurfDesk Web Application"

    # Navigate to dist directory
    cd dist

    # Get local IP address for network access
    LOCAL_IP=$(hostname -I | awk '{print $1}' 2>/dev/null || echo "127.0.0.1")

    print_status "Starting web server..."
    echo -e "${CYAN}📱 Local access: http://localhost:8080${NC}"
    echo -e "${CYAN}🌐 Network access: http://$LOCAL_IP:8080${NC}"
    echo -e "${CYAN}📋 Press Ctrl+C to stop the server${NC}"
    echo ""

    # Start the web server
    if command -v python3 &> /dev/null; then
        python3 -m http.server 8080
    elif command -v python &> /dev/null; then
        python -m http.server 8080
    elif command -v npx &> /dev/null; then
        npx serve -p 8080
    else
        print_error "No suitable web server found"
        exit 1
    fi
}

# Development mode
dev_mode() {
    print_header "🔧 Development Mode"

    # Watch for changes and rebuild
    print_status "Watching for changes..."

    while true; do
        echo -e "${YELLOW}🔄 Rebuilding web application...${NC}"
        build_web
        echo -e "${GREEN}✅ Build complete. Server running at http://localhost:8080${NC}"
        echo -e "${CYAN}💡 Press Ctrl+C to stop watching${NC}"

        # Wait for changes (simple file watching)
        inotifywait -e modify,create,delete -r surfdesk-web/src/ 2>/dev/null || sleep 5
    done
}

# Clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    rm -rf dist/
    rm -rf surfdesk-web/pkg/
    rm -rf surfdesk-web/target/
    print_status "✅ Clean complete"
}

# Show help
show_help() {
    echo "🌐 SurfDesk Web Application Runner"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  build     Build the web application"
    echo "  serve     Build and serve the web application"
    echo "  dev       Development mode with auto-reload"
    echo "  clean     Clean build artifacts"
    echo "  help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 serve      # Build and serve on http://localhost:8080"
    echo "  $0 dev        # Development mode with auto-reload"
    echo "  $0 clean      # Clean all build files"
}

# Main execution
main() {
    print_header "🏄‍♂️ SurfDesk Web Application"

    check_directory
    check_dependencies

    case "${1:-serve}" in
        "build")
            build_web
            ;;
        "serve")
            build_web
            serve_web
            ;;
        "dev")
            dev_mode
            ;;
        "clean")
            clean
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            print_error "Unknown command: $1"
            show_help
            exit 1
            ;;
    esac
}

# Handle script interruption
trap 'print_warning "Script interrupted by user"; exit 0' INT

# Run main function
main "$@"
