#!/bin/bash

# Turso Database Setup Script for SurfDesk
# This script helps set up Turso database for SurfDesk

set -e

echo "🌊 SurfDesk Turso Setup Script"
echo "================================"

# Check if Turso CLI is installed
if ! command -v turso &> /dev/null; then
    echo "❌ Turso CLI not found. Installing..."
    curl -sSfL https://get.tur.so/install.sh | bash
    echo "✅ Turso CLI installed successfully"
else
    echo "✅ Turso CLI found"
fi

# Check if user is logged in to Turso
if ! turso auth whoami &> /dev/null; then
    echo "🔐 Please log in to Turso:"
    turso auth login
else
    echo "✅ Already logged in to Turso"
fi

# Database name
DB_NAME=${1:-surfdesk}
echo "📊 Using database name: $DB_NAME"

# Create database if it doesn't exist
if turso db list | grep -q "$DB_NAME"; then
    echo "✅ Database '$DB_NAME' already exists"
else
    echo "📊 Creating database '$DB_NAME'..."
    turso db create "$DB_NAME"
    echo "✅ Database created successfully"
fi

# Get database URL
DB_URL=$(turso db show "$DB_NAME" --url)
echo "🔗 Database URL: $DB_URL"

# Create auth token
echo "🔑 Creating authentication token..."
AUTH_TOKEN=$(turso auth tokens create "$DB_NAME")
echo "✅ Auth token created"

# Create config directory
mkdir -p config/turso

# Create configuration file
CONFIG_FILE="config/turso/turso.env"
if [ -f "$CONFIG_FILE" ]; then
    echo "⚠️  Configuration file already exists. Backing up..."
    cp "$CONFIG_FILE" "$CONFIG_FILE.backup.$(date +%s)"
fi

cat > "$CONFIG_FILE" << EOF
# Turso Database Configuration for SurfDesk
# Generated on $(date)

# Turso Database URL
TURSO_URL=$DB_URL

# Turso Authentication Token
TURSO_AUTH_TOKEN=$AUTH_TOKEN

# Turso Database Name
TURSO_DATABASE_NAME=$DB_NAME

# Enable Turso (set to "true" to use Turso)
ENABLE_TURSO=true

# Optional: Custom SQLite path (fallback)
SQLITE_PATH=./data/surfdesk.db
EOF

echo "✅ Configuration saved to $CONFIG_FILE"

# Test connection
echo "🧪 Testing database connection..."
if turso db shell "$DB_NAME" "SELECT 1;" &> /dev/null; then
    echo "✅ Database connection successful"
else
    echo "❌ Database connection failed"
    exit 1
fi

# Instructions
echo ""
echo "🎉 Turso setup complete!"
echo ""
echo "Next steps:"
echo "1. Review the configuration in $CONFIG_FILE"
echo "2. Run SurfDesk with Turso feature:"
echo "   cargo run --features turso"
echo ""
echo "To switch back to SQLite, set ENABLE_TURSO=false in $CONFIG_FILE"
echo ""
echo "For more information, see config/turso/README.md"
```

<file_path>
solana_account_studio/scripts/switch-db.sh
</file_path>

<edit_description>
Create database switching script
</edit_description>
```bash
#!/bin/bash

# Database Backend Switch Script for SurfDesk
# This script helps switch between SQLite and Turso databases

set -e

echo "🔄 SurfDesk Database Backend Switch"
echo "==================================="

# Configuration
CONFIG_FILE="config/turso/turso.env"
SQLITE_DEFAULT_PATH="./data/surfdesk.db"

# Function to show current backend
show_current() {
    if [ -f "$CONFIG_FILE" ]; then
        source "$CONFIG_FILE"
        if [ "${ENABLE_TURSO:-false}" = "true" ]; then
            echo "📍 Current backend: Turso"
            echo "   Database: ${TURSO_DATABASE_NAME:-not set}"
            echo "   URL: ${TURSO_URL:-not set}"
        else
            echo "📍 Current backend: SQLite"
            echo "   Path: ${SQLITE_PATH:-$SQLITE_DEFAULT_PATH}"
        fi
    else
        echo "📍 Current backend: SQLite (default)"
        echo "   Path: $SQLITE_DEFAULT_PATH"
        echo ""
        echo "ℹ️  To configure Turso, run: ./scripts/setup-turso.sh"
    fi
}

# Function to switch to Turso
switch_to_turso() {
    echo "🌊 Switching to Turso..."

    if [ ! -f "$CONFIG_FILE" ]; then
        echo "❌ Turso configuration not found."
        echo "   Please run: ./scripts/setup-turso.sh"
        exit 1
    fi

    # Backup current SQLite database if it exists
    if [ -f "$SQLITE_DEFAULT_PATH" ]; then
        echo "💾 Backing up current SQLite database..."
        cp "$SQLITE_DEFAULT_PATH" "${SQLITE_DEFAULT_PATH}.backup.$(date +%s)"
        echo "✅ Backup created"
    fi

    # Update configuration
    sed -i.bak 's/ENABLE_TURSO=.*/ENABLE_TURSO=true/' "$CONFIG_FILE"
    echo "✅ Switched to Turso backend"

    # Test Turso connection
    source "$CONFIG_FILE"
    if command -v turso &> /dev/null; then
        echo "🧪 Testing Turso connection..."
        if turso db shell "$TURSO_DATABASE_NAME" "SELECT 1;" &> /dev/null; then
            echo "✅ Turso connection successful"
        else
            echo "⚠️  Turso connection test failed"
            echo "   Please check your configuration in $CONFIG_FILE"
        fi
    fi

    echo ""
    echo "🚀 Run with: cargo run --features turso"
}

# Function to switch to SQLite
switch_to_sqlite() {
    echo "📊 Switching to SQLite..."

    # Create config directory if it doesn't exist
    mkdir -p "$(dirname "$CONFIG_FILE")"

    # Update or create configuration
    if [ -f "$CONFIG_FILE" ]; then
        sed -i.bak 's/ENABLE_TURSO=.*/ENABLE_TURSO=false/' "$CONFIG_FILE"
    else
        cat > "$CONFIG_FILE" << EOF
# Database Configuration
ENABLE_TURSO=false
SQLITE_PATH=$SQLITE_DEFAULT_PATH
EOF
    fi

    # Ensure SQLite directory exists
    mkdir -p "$(dirname "$SQLITE_DEFAULT_PATH")"

    echo "✅ Switched to SQLite backend"
    echo "   Database path: $SQLITE_DEFAULT_PATH"

    echo ""
    echo "🚀 Run with: cargo run"
}

# Function to show help
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  status     Show current database backend"
    echo "  turso      Switch to Turso backend"
    echo "  sqlite     Switch to SQLite backend"
    echo "  help       Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 status    # Show current backend"
    echo "  $0 turso     # Switch to Turso"
    echo "  $0 sqlite    # Switch to SQLite"
}

# Main script logic
case "${1:-status}" in
    "status"|"show")
        show_current
        ;;
    "turso")
        switch_to_turso
        ;;
    "sqlite")
        switch_to_sqlite
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
```

Now let me make the scripts executable and push them:
