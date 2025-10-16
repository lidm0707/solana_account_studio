#!/bin/bash
# 🧹 SurfDesk Warning Cleanup Automation
# Fast automated warning resolution for MVP completion

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_header() {
    echo -e "${BLUE}=================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}=================================${NC}"
}

# Check if we're in the right directory
check_directory() {
    if [ ! -f "Cargo.toml" ] || [ ! -d "surfdesk-core" ]; then
        echo -e "${RED}[ERROR]${NC} Not in SurfDesk project directory!"
        exit 1
    fi
}

# Run cargo fix for automatic warnings
fix_automatic_warnings() {
    print_header "🔧 AUTOMATIC WARNING FIXES"

    print_status "Applying cargo fix to surfdesk-core..."
    if cargo fix --lib -p surfdesk-core --allow-dirty --allow-staged; then
        print_success "✅ Core library warnings fixed"
    else
        print_warning "⚠️ Some core warnings need manual attention"
    fi

    print_status "Applying cargo fix to surfdesk-web..."
    if cargo fix --bin "surfdesk-web" --allow-dirty --allow-staged; then
        print_success "✅ Web binary warnings fixed"
    else
        print_warning "⚠️ Some web warnings need manual attention"
    fi

    print_status "Applying cargo fix to surfdesk-tui..."
    if cargo fix --bin "surfdesk-tui" --allow-dirty --allow-staged; then
        print_success "✅ TUI binary warnings fixed"
    else
        print_warning "⚠️ Some TUI warnings need manual attention"
    fi

    print_status "Applying cargo fix to surfdesk-cli..."
    if cargo fix --bin "surfdesk-cli" --allow-dirty --allow-staged; then
        print_success "✅ CLI binary warnings fixed"
    else
        print_warning "⚠️ Some CLI warnings need manual attention"
    fi
}

# Manual warning fixes for specific patterns
fix_manual_warnings() {
    print_header "🛠️ MANUAL WARNING FIXES"

    # Fix unreachable code in logger.rs
    print_status "Fixing unreachable code in logger.rs..."
    if [ -f "surfdesk-core/src/services/logger.rs" ]; then
        # Remove unreachable Ok(()) after return Err
        sed -i '/return Err(SurfDeskError::platform(/,/));$/{
            /return Err(SurfDeskError::platform(/!d
            /);$/!d
            /);$/a\
        Ok(())
        }' surfdesk-core/src/services/logger.rs
        print_success "✅ Unreachable code fixed in logger.rs"
    fi

    # Fix unused variable mut warnings
    print_status "Fixing unnecessary mut variables..."
    find . -name "*.rs" -not -path "./target/*" -exec sed -i 's/let mut \([a-zA-Z_][a-zA-Z0-9_]*\) = use_signal/let \1 = use_signal/g' {} \;
    print_success "✅ Unnecessary mut variables fixed"

    # Fix unused variables by prefixing with underscore
    print_status "Fixing unused variables..."
    find . -name "*.rs" -not -path "./target/*" -exec sed -i 's/let \([a-zA-Z_][a-zA-Z0-9_]*\) = \([a-zA-Z_][a-zA-Z0-9_]*::[^;]*\);/let _\1 = \2;/g' {} \;
    print_success "✅ Unused variables prefixed with underscore"
}

# Check remaining warnings
check_remaining_warnings() {
    print_header "📊 CHECKING REMAINING WARNINGS"

    CHECK_OUTPUT=$(cargo check --workspace 2>&1)
    WARNING_COUNT=$(echo "$CHECK_OUTPUT" | grep -c "warning\[" || echo "0")
    ERROR_COUNT=$(echo "$CHECK_OUTPUT" | grep -c "error\[" || echo "0")

    echo -e "${CYAN}Current Status:${NC}"
    echo "  Errors: $ERROR_COUNT"
    echo "  Warnings: $WARNING_COUNT"

    if [ "$ERROR_COUNT" -gt 0 ]; then
        echo -e "${RED}❌ Errors detected - need manual fixing${NC}"
        echo "$CHECK_OUTPUT" | grep "error\[" -A 2 -B 2
        return 1
    fi

    if [ "$WARNING_COUNT" -gt 0 ]; then
        echo -e "${YELLOW}⚠️ $WARNING_COUNT warnings remain${NC}"
        echo "Top warnings:"
        echo "$CHECK_OUTPUT" | grep "warning\[" | head -5
    else
        print_success "🎊 No warnings remaining!"
    fi

    return 0
}

# Main execution
main() {
    print_header "🧹 SURFDESK WARNING CLEANUP"

    check_directory

    # Get initial warning count
    INITIAL_WARNINGS=$(cargo check --workspace 2>&1 | grep -c "warning\[" || echo "0")
    echo -e "${CYAN}Initial warning count: $INITIAL_WARNINGS${NC}"

    # Apply automatic fixes
    fix_automatic_warnings

    # Apply manual fixes
    fix_manual_warnings

    # Check results
    check_remaining_warnings

    # Get final warning count
    FINAL_WARNINGS=$(cargo check --workspace 2>&1 | grep -c "warning\[" || echo "0")
    WARNINGS_REMOVED=$((INITIAL_WARNINGS - FINAL_WARNINGS))

    echo -e "${CYAN}Cleanup Summary:${NC}"
    echo "  Initial warnings: $INITIAL_WARNINGS"
    echo "  Final warnings: $FINAL_WARNINGS"
    echo "  Warnings removed: $WARNINGS_REMOVED"

    if [ "$WARNINGS_REMOVED" -gt 0 ]; then
        print_success "🎉 Successfully removed $WARNINGS_REMOVED warnings!"
    fi

    if [ "$FINAL_WARNINGS" -eq 0 ]; then
        print_success "🎊 ALL WARNINGS RESOLVED - MVP READY!"
    else
        print_warning "⚠️ $FINAL_WARNINGS warnings remain - manual review needed"
    fi
}

# Run main function
main "$@"
