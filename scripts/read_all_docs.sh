#!/bin/bash
# 📚 Read All Documentation Shell Script for SurfDesk
#
# This script provides a simple interface for analyzing all documentation files.
#
# Usage: ./read_all_docs.sh [options]
#
# Examples:
#   ./read_all_docs.sh                    # Summary of all docs
#   ./read_all_docs.sh --technical        # Focus on technical docs

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Project root detection
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Function to show help
show_help() {
    cat << EOF
📚 All Documentation Reader

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -t, --technical         Focus on technical documentation
    -a, --agent             Focus on agent-related documentation
    -p, --product           Focus on product documentation
    --project-root DIR      Project root directory (default: auto-detect)

EXAMPLES:
    $0                          # Summary of all docs
    $0 --technical              # Focus on technical docs
    $0 --agent                  # Focus on agent-related docs
    $0 --product                # Focus on product docs

EOF
}

# Function to print colored header
print_header() {
    echo -e "${CYAN}📚 All Documentation Reader${NC}"
    echo -e "${BLUE}Project: $PROJECT_ROOT${NC}"
    echo -e "Avoid using #![allow(dead_code)]"
    echo -e "Follow best practices"
    echo -e "Use the Onion Architecture"
    echo -e "Understand SOLID principles"
    echo -e "Traits are mostly used in the connection layer"
    echo -e "When a cycle is completed: git push"
    echo -e "Reflect, rethink, and continue the next iteration"
    echo -e "Work step-by-step, so you can safely pause before hitting limits"
    echo -e "No mock data"
    echo ""
}

# Function to find all documentation files
find_all_docs() {
    local docs=()

    # Root level markdown files
    for file in "$PROJECT_ROOT"/*.md; do
        if [[ -f "$file" ]]; then
            docs+=("$(basename "$file")")
        fi
    done

    # Docs directory
    if [[ -d "$PROJECT_ROOT/docs" ]]; then
        while IFS= read -r -d '' file; do
            local rel_path="${file#$PROJECT_ROOT/}"
            docs+=("$rel_path")
        done < <(find "$PROJECT_ROOT/docs" -name "*.md" -type f -print0 2>/dev/null | sort -z)
    fi

    printf '%s\n' "${docs[@]}"
}

# Function to categorize file
categorize_file() {
    local file="$1"
    local filename=$(basename "$file")

    case "$filename" in
        AGENT.md|order_agent.md)
            echo "agent"
            ;;
        DIOXUS.md|DATABASE.md|DIESEL_TO_TURSO.md)
            echo "technical"
            ;;
        PRODUCT.md|README.md|ROADMAP.md)
            echo "product"
            ;;
        FLOWTOOL.md|MVP_PROCESS.md|algorithm.md)
            echo "workflow"
            ;;
        ERROR_RUNTIME.md)
            echo "error"
            ;;
        SURFPOOL.md|BREAKTOOL.md)
            echo "infrastructure"
            ;;
        *)
            if [[ "$filename" =~ agent ]]; then
                echo "agent"
            elif [[ "$filename" =~ (dio|database|turso|diesel) ]]; then
                echo "technical"
            elif [[ "$filename" =~ (product|readme|roadmap) ]]; then
                echo "product"
            elif [[ "$filename" =~ (flow|process|algorithm) ]]; then
                echo "workflow"
            elif [[ "$file" =~ error ]]; then
                echo "error"
            elif [[ "$file" =~ (pool|break) ]]; then
                echo "infrastructure"
            elif [[ "$file" =~ (status|mvp|platform) ]]; then
                echo "status"
            else
                echo "other"
            fi
            ;;
    esac
}

# Function to get file stats
get_file_stats() {
    local file="$1"
    local lines=0
    local size=0

    if [[ -f "$file" ]]; then
        lines=$(wc -l < "$file" 2>/dev/null || echo "0")
        size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo "0")
    fi

    echo "$lines:$size"
}

# Function to extract key insights
extract_insights() {
    local file="$1"
    local count=0

    while IFS= read -r line && [[ $count -lt 5 ]]; do
        local trimmed=$(echo "$line" | sed 's/^[[:space:]]*//')
        if [[ "$trimmed" =~ ^(✅|❌|⚠️|🎯|🚀|🏆|📊|🔧|🌊)[[:space:]] ]]; then
            echo "  • $trimmed"
            count=$((count + 1))
        fi
    done < "$file"
}

# Function to assess project status
assess_project_status() {
    local compilation="unknown"
    local phase="unknown"
    local framework=""

    if [[ -f "$PROJECT_ROOT/AGENT.md" ]]; then
        if grep -q "ZERO COMPILATION ERRORS" "$PROJECT_ROOT/AGENT.md" 2>/dev/null; then
            compilation="success"
        elif grep -q "compilation errors" "$PROJECT_ROOT/AGENT.md" 2>/dev/null; then
            compilation="has_errors"
        fi

        if grep -q "FEATURE DEVELOPMENT" "$PROJECT_ROOT/AGENT.md" 2>/dev/null; then
            phase="feature_development"
        elif grep -q "COMPILATION RESOLUTION" "$PROJECT_ROOT/AGENT.md" 2>/dev/null; then
            phase="compilation_resolution"
        fi
    fi

    if [[ -f "$PROJECT_ROOT/DIOXUS.md" ]] && grep -q "Dioxus 0.6" "$PROJECT_ROOT/DIOXUS.md" 2>/dev/null; then
        framework="Dioxus 0.6+"
    fi

    echo "$compilation:$phase:$framework"
}

# Function to get the first non-empty line (title or heading)
get_first_line() {
    local file="$1"
    if [[ -f "$file" ]]; then
        # ดึงบรรทัดแรกที่ไม่ว่าง
        awk 'NF {print; exit}' "$file"
    else
        echo ""
    fi
}

# Function to generate summary
generate_summary() {
    local filter_category=""
    if [[ -n "${1:-}" ]]; then
        filter_category="$1"
    fi

    echo -e "${CYAN}🌊 SurfDesk Project Documentation Summary${NC}"
    echo -e "${BLUE}==================================================${NC}"
    echo ""

    # Get all docs
    local all_docs
    readarray -t all_docs < <(find_all_docs)

    local total_files=0
    local total_lines=0
    local total_size=0
    declare -A category_counts

    # Process each file
    for doc in "${all_docs[@]}"; do
        local file_path="$PROJECT_ROOT/$doc"
        if [[ ! -f "$file_path" ]]; then
            continue
        fi

        # Get first line for context
        local first_line=$(get_first_line "$file_path")

        # Print file info
        echo -e "${YELLOW}📄 File:${NC} $doc"
        echo "   Path: $file_path"
        echo "   First Line: $first_line"
        echo ""

        local category=$(categorize_file "$doc")
        if [[ -n "$filter_category" && "$category" != "$filter_category" ]]; then
            continue
        fi

        local stats=$(get_file_stats "$file_path")
        local lines=$(echo "$stats" | cut -d: -f1)
        local size=$(echo "$stats" | cut -d: -f2)

        total_files=$((total_files + 1))
        total_lines=$((total_lines + lines))
        total_size=$((total_size + size))

        # Update category counts
        local current_count=${category_counts[$category]:-0}
        category_counts[$category]=$((current_count + 1))
    done

    # Basic stats
    echo -e "${GREEN}📊 Documentation Statistics:${NC}"
    echo "  • Total files: $total_files"
    echo "  • Total lines: $(printf "%'d" $total_lines)"
    echo "  • Total size: $(printf "%'d" $total_size) bytes"
    echo ""

    # Project status
    local status_info=$(assess_project_status)
    echo -e "${GREEN}🎯 Project Status:${NC}"
    echo "  • Compilation: $(echo "$status_info" | cut -d: -f1)"
    echo "  • Development Phase: $(echo "$status_info" | cut -d: -f2)"
    local framework=$(echo "$status_info" | cut -d: -f3)
    if [[ -n "$framework" ]]; then
        echo "  • Framework: $framework"
    fi
    echo ""

    # Categories
    echo -e "${GREEN}📁 Documentation Categories:${NC}"
    for category in "${!category_counts[@]}"; do
        local count=${category_counts[$category]}
        echo "  • ${category^}: $count files"
    done
    echo ""

    # Key insights
    echo -e "${GREEN}💡 Key Insights:${NC}"
    local insight_count=0
    for doc in "${all_docs[@]}"; do
        if [[ $insight_count -ge 10 ]]; then
            break
        fi

        local file_path="$PROJECT_ROOT/$doc"
        if [[ ! -f "$file_path" ]]; then
            continue
        fi

        local category=$(categorize_file "$doc")
        if [[ -n "$filter_category" && "$category" != "$filter_category" ]]; then
            continue
        fi

        local insights=$(extract_insights "$file_path")
        while IFS= read -r insight && [[ $insight_count -lt 10 ]]; do
            echo "$insight"
            insight_count=$((insight_count + 1))
        done <<< "$insights"
    done
    echo ""

    # Recommendations
    echo -e "${GREEN}🎯 Recommendations:${NC}"
    echo "  • Review and address action items across documentation"
    echo "  • Complete Diesel to Turso migration following the migration guide"
    echo "  • Keep documentation updated with development progress"
    echo "  • Use the read_docs.sh script for easy documentation access"
    echo "  • Review AGENT.md regularly for current development priorities"
    echo ""
}

# Main script execution
main() {
    local filter_category=""

    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -t|--technical)
                filter_category="technical"
                shift
                ;;
            -a|--agent)
                filter_category="agent"
                shift
                ;;
            -p|--product)
                filter_category="product"
                shift
                ;;
            --project-root)
                PROJECT_ROOT="$2"
                shift 2
                ;;
            -*)
                echo -e "${RED}❌ Unknown option: $1${NC}" >&2
                echo "Use --help for usage information." >&2
                exit 1
                ;;
            *)
                echo -e "${RED}❌ Unknown argument: $1${NC}" >&2
                echo "Use --help for usage information." >&2
                exit 1
                ;;
        esac
    done

    # Show header
    print_header

    # Generate summary
    generate_summary "$filter_category"
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
