#!/bin/bash
# 📚 Documentation Reader Shell Script for SurfDesk
#
# This script provides a comprehensive interface for reading and analyzing
# markdown documentation files in the SurfDesk project using pure shell.
#
# Usage: ./read_docs.sh [options] [file]
#
# Examples:
#   ./read_docs.sh AGENT.md                    # Read AGENT.md
#   ./read_docs.sh --list                       # List all available docs
#   ./read_docs.sh --search "database"          # Search for database-related content
#   ./read_docs.sh --summary                    # Get summary of all docs
#   ./read_docs.sh DIOXUS.md --section "routing" # Read specific section

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

# Documentation files
ROOT_DOCS=(
    "AGENT.md"
    "FLOWTOOL.md"
    "ERROR_RUNTIME.md"
    "PRODUCT.md"
    "SURFPOOL.md"
    "DIOXUS.md"
    "README.md"
    "ROADMAP.md"
    "DATABASE.md"
)

# Function to show help
show_help() {
    cat << EOF
📚 SurfDesk Documentation Reader

USAGE:
    $0 [OPTIONS] [FILE]

OPTIONS:
    -h, --help              Show this help message
    -l, --list              List all available documentation files
    -s, --search QUERY      Search for content across all documentation
    --summary               Generate summary of all documentation
    --section SECTION       Read specific section from file
    --format FORMAT         Output format: readable, summary (default: readable)
    --project-root DIR      Project root directory (default: auto-detect)

EXAMPLES:
    $0 AGENT.md                           # Read AGENT.md
    $0 --list                             # List all available docs
    $0 --search "database"                # Search for database-related content
    $0 --summary                          # Get summary of all docs
    $0 DIOXUS.md --section "routing"      # Read specific section
    $0 --format summary AGENT.md          # Output as summary

AVAILABLE FILES:
    • AGENT.md - Development agent guidelines
    • FLOWTOOL.md - Workflow tool documentation
    • ERROR_RUNTIME.md - Runtime error handling
    • PRODUCT.md - Product specifications
    • SURFPOOL.md - SurfPool integration
    • DIOXUS.md - Dioxus framework guide
    • README.md - Project overview
    • ROADMAP.md - Development roadmap
    • DATABASE.md - Database architecture

EOF
}

# Function to print colored header
print_header() {
    echo -e "${CYAN}📚 SurfDesk Documentation Reader${NC}"
    echo -e "${BLUE}Project: $PROJECT_ROOT${NC}"
    echo ""
}

# Function to list available documentation files
list_docs() {
    echo -e "${GREEN}📁 Available Documentation Files:${NC}"

    # Check root directory docs
    for doc in "${ROOT_DOCS[@]}"; do
        if [[ -f "$PROJECT_ROOT/$doc" ]]; then
            local size=$(stat -f%z "$PROJECT_ROOT/$doc" 2>/dev/null || stat -c%s "$PROJECT_ROOT/$doc" 2>/dev/null || echo "0")
            local lines=$(wc -l < "$PROJECT_ROOT/$doc" 2>/dev/null || echo "0")
            echo -e "  • ${GREEN}$doc${NC} ($lines lines, $size bytes)"
        fi
    done

    # Check docs subdirectory
    if [[ -d "$PROJECT_ROOT/docs" ]]; then
        while IFS= read -r -d '' file; do
            local rel_path="${file#$PROJECT_ROOT/}"
            local size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo "0")
            local lines=$(wc -l < "$file" 2>/dev/null || echo "0")
            echo -e "  • ${YELLOW}$rel_path${NC} ($lines lines, $size bytes)"
        done < <(find "$PROJECT_ROOT/docs" -name "*.md" -type f -print0 2>/dev/null | sort -z)
    fi
}

# Function to resolve file path
resolve_file_path() {
    local file="$1"

    # Try direct path
    if [[ -f "$PROJECT_ROOT/$file" ]]; then
        echo "$PROJECT_ROOT/$file"
        return 0
    fi

    # Try in docs directory
    if [[ -f "$PROJECT_ROOT/docs/$file" ]]; then
        echo "$PROJECT_ROOT/docs/$file"
        return 0
    fi

    # Try with .md extension
    if [[ ! "$file" == *.md ]]; then
        if [[ -f "$PROJECT_ROOT/$file.md" ]]; then
            echo "$PROJECT_ROOT/$file.md"
            return 0
        fi
        if [[ -f "$PROJECT_ROOT/docs/$file.md" ]]; then
            echo "$PROJECT_ROOT/docs/$file.md"
            return 0
        fi
    fi

    return 1
}

# Function to extract sections from markdown
extract_sections() {
    local file="$1"
    local in_code_block=false

    while IFS= read -r line; do
        # Track code blocks
        if [[ "$line" =~ ^\`\`\` ]]; then
            in_code_block=$(! $in_code_block)
            continue
        fi

        # Skip lines in code blocks
        if [[ "$in_code_block" == true ]]; then
            continue
        fi

        # Extract headers
        if [[ "$line" =~ ^#+[[:space:]]+(.+)$ ]]; then
            local header="${BASH_REMATCH[1]}"
            local level=$(echo "$line" | sed 's/^\(#*\).*/\1/' | wc -c)
            level=$((level - 1))

            # Extract emoji if present
            local emoji=""
            if [[ "$header" =~ ([🌊📋🏗️🚀🎨🔄🔧📦📚🎯💼🏆📊🗺️🎭]) ]]; then
                emoji="${BASH_REMATCH[1]} "
            fi

            local indent=""
            for ((i=1; i<level; i++)); do
                indent="  $indent"
            done

            echo "${indent}${emoji}${header}"
        fi
    done < "$file"
}

# Function to extract key points
extract_key_points() {
    local file="$1"
    local in_code_block=false

    while IFS= read -r line; do
        # Track code blocks
        if [[ "$line" =~ ^\`\`\` ]]; then
            in_code_block=$(! $in_code_block)
            continue
        fi

        # Skip lines in code blocks
        if [[ "$in_code_block" == true ]]; then
            continue
        fi

        # Look for bullet points with emojis or special markers
        local trimmed=$(echo "$line" | sed 's/^[[:space:]]*//')
        if [[ "$trimmed" =~ ^(✅|❌|⚠️|🎯|🚀|🏆|📊|🔧|🌊)[[:space:]] ]]; then
            echo "  • $trimmed"
        fi

        # Look for numbered lists with important content
        if [[ "$trimmed" =~ ^[0-9]+\.[[:space:]]+.{20,} ]]; then
            echo "  • $trimmed"
        fi
    done < "$file"
}

# Function to search content
search_content() {
    local query="$1"
    local found=false

    echo -e "${GREEN}🔍 Search results for '${YELLOW}$query${GREEN}':${NC}"
    echo ""

    # Search in root docs
    for doc in "${ROOT_DOCS[@]}"; do
        if [[ -f "$PROJECT_ROOT/$doc" ]]; then
            local matches=$(grep -n -i --ignore-case "$query" "$PROJECT_ROOT/$doc" 2>/dev/null || true)
            if [[ -n "$matches" ]]; then
                echo -e "${BLUE}📄 $doc${NC}"
                echo "$matches" | head -5 | while IFS=: read -r line_num content; do
                    echo "  Line $line_num: $content"
                done
                echo ""
                found=true
            fi
        fi
    done

    # Search in docs directory
    if [[ -d "$PROJECT_ROOT/docs" ]]; then
        while IFS= read -r -d '' file; do
            local rel_path="${file#$PROJECT_ROOT/}"
            local matches=$(grep -n -i --ignore-case "$query" "$file" 2>/dev/null || true)
            if [[ -n "$matches" ]]; then
                echo -e "${YELLOW}📄 $rel_path${NC}"
                echo "$matches" | head -5 | while IFS=: read -r line_num content; do
                    echo "  Line $line_num: $content"
                done
                echo ""
                found=true
            fi
        done < <(find "$PROJECT_ROOT/docs" -name "*.md" -type f -print0 2>/dev/null)
    fi

    if [[ "$found" == false ]]; then
        echo -e "${RED}❌ No results found for '${query}'${NC}"
    fi
}

# Function to get file metadata
get_file_metadata() {
    local file="$1"
    local file_size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo "0")
    local line_count=$(wc -l < "$file" 2>/dev/null || echo "0")
    local word_count=$(wc -w < "$file" 2>/dev/null || echo "0")
    local last_mod=$(stat -f%Sm -t%Y-%m-%d "$file" 2>/dev/null || stat -c%y "$file" 2>/dev/null | cut -d' ' -f1 || echo "unknown")

    echo "📊 $line_count lines | $word_count words | $file_size bytes | Modified: $last_mod"
}

# Function to format readable output
format_readable() {
    local file="$1"
    local rel_path="${file#$PROJECT_ROOT/}"

    echo -e "${CYAN}📄 $rel_path${NC}"
    get_file_metadata "$file"
    echo ""

    # Table of contents
    echo -e "${GREEN}## 📋 Table of Contents${NC}"
    local sections=$(extract_sections "$file")
    if [[ -n "$sections" ]]; then
        echo "$sections"
    else
        echo "  No sections found"
    fi
    echo ""

    # Key points
    echo -e "${GREEN}## 💡 Key Points${NC}"
    local key_points=$(extract_key_points "$file")
    if [[ -n "$key_points" ]]; then
        echo "$key_points"
    else
        echo "  No key points found"
    fi
    echo ""

    # Content (first 50 lines)
    echo -e "${GREEN}## 📖 Content Preview${NC}"
    head -50 "$file" | while IFS= read -r line; do
        echo "$line"
    done

    # Show if content was truncated
    local total_lines=$(wc -l < "$file")
    if [[ $total_lines -gt 50 ]]; then
        echo ""
        echo -e "${YELLOW}... ($((total_lines - 50)) more lines)${NC}"
        echo -e "${YELLOW}Use --section to read specific sections${NC}"
    fi
}

# Function to format summary output
format_summary() {
    local file="$1"
    local rel_path="${file#$PROJECT_ROOT/}"

    echo -e "${CYAN}📄 $rel_path${NC}"
    get_file_metadata "$file"

    # Main sections (level 1 and 2)
    echo -e "${GREEN}🏗️ Main Sections:${NC}"
    local main_sections=$(extract_sections "$file" | grep -E '^[^[:space:]]|^[[:space:]]{2}' | head -5)
    if [[ -n "$main_sections" ]]; then
        echo "$main_sections" | sed 's/^/  /'
    else
        echo "  No main sections found"
    fi

    # Code blocks count
    local code_blocks=$(grep -c '^```' "$file" 2>/dev/null || echo "0")
    code_blocks=$((code_blocks / 2))
    if [[ $code_blocks -gt 0 ]]; then
        echo -e "${GREEN}💻 Code:${NC} $code_blocks code blocks"
    fi
}

# Function to get section content
get_section_content() {
    local file="$1"
    local section_name="$2"
    local in_target_section=false
    local section_level=0
    local current_level=0
    local in_code_block=false

    while IFS= read -r line; do
        # Track code blocks
        if [[ "$line" =~ ^\`\`\` ]]; then
            in_code_block=$(! $in_code_block)
        fi

        # Skip lines in code blocks for header detection
        if [[ "$in_code_block" == true ]]; then
            if [[ "$in_target_section" == true ]]; then
                echo "$line"
            fi
            continue
        fi

        # Check if this is a header
        if [[ "$line" =~ ^#+[[:space:]]+(.+)$ ]]; then
            local header="${BASH_REMATCH[1]}"
            current_level=$(echo "$line" | sed 's/^\(#*\).*/\1/' | wc -c)
            current_level=$((current_level - 1))

            # Check if this matches our target section
            if [[ "${header,,}" == *"${section_name,,}"* ]]; then
                in_target_section=true
                section_level=$current_level
                echo "$line"
                continue
            fi

            # Check if we're leaving the target section
            if [[ "$in_target_section" == true && $current_level -le $section_level ]]; then
                break
            fi
        fi

        # Print content if we're in the target section
        if [[ "$in_target_section" == true ]]; then
            echo "$line"
        fi
    done < "$file"

    if [[ "$in_target_section" == false ]]; then
        echo -e "${RED}❌ Section '$section_name' not found${NC}" >&2
        return 1
    fi
}

# Function to generate summary of all docs
generate_summary() {
    echo -e "${GREEN}📊 Documentation Summary:${NC}"
    echo ""

    local total_files=0
    local total_size=0
    local total_lines=0
    local total_sections=0
    local total_code_blocks=0

    # Process root docs
    for doc in "${ROOT_DOCS[@]}"; do
        if [[ -f "$PROJECT_ROOT/$doc" ]]; then
            local file="$PROJECT_ROOT/$doc"
            local size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo "0")
            local lines=$(wc -l < "$file" 2>/dev/null || echo "0")
            local sections=$(extract_sections "$file" | wc -l || echo "0")
            local code_blocks=$(grep -c '^```' "$file" 2>/dev/null || echo "0")
            code_blocks=$((code_blocks / 2))

            total_files=$((total_files + 1))
            total_size=$((total_size + size))
            total_lines=$((total_lines + lines))
            total_sections=$((total_sections + sections))
            total_code_blocks=$((total_code_blocks + code_blocks))

            echo -e "  • ${GREEN}$doc${NC}: $lines lines, $sections sections, $code_blocks code blocks"
        fi
    done

    # Process docs directory
    if [[ -d "$PROJECT_ROOT/docs" ]]; then
        while IFS= read -r -d '' file; do
            local rel_path="${file#$PROJECT_ROOT/}"
            local size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo "0")
            local lines=$(wc -l < "$file" 2>/dev/null || echo "0")
            local sections=$(extract_sections "$file" | wc -l || echo "0")
            local code_blocks=$(grep -c '^```' "$file" 2>/dev/null || echo "0")
            code_blocks=$((code_blocks / 2))

            total_files=$((total_files + 1))
            total_size=$((total_size + size))
            total_lines=$((total_lines + lines))
            total_sections=$((total_sections + sections))
            total_code_blocks=$((total_code_blocks + code_blocks))

            echo -e "  • ${YELLOW}$rel_path${NC}: $lines lines, $sections sections, $code_blocks code blocks"
        done < <(find "$PROJECT_ROOT/docs" -name "*.md" -type f -print0 2>/dev/null | sort -z)
    fi

    echo ""
    echo -e "${BLUE}📈 Totals:${NC}"
    echo "  Total files: $total_files"
    echo "  Total size: $total_size bytes"
    echo "  Total lines: $total_lines"
    echo "  Total sections: $total_sections"
    echo "  Total code blocks: $total_code_blocks"
}

# Main script execution
main() {
    # Parse command line arguments
    local show_list=false
    local search_query=""
    local show_summary=false
    local section_name=""
    local output_format="readable"
    local target_file=""

    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -l|--list)
                show_list=true
                shift
                ;;
            -s|--search)
                search_query="$2"
                shift 2
                ;;
            --summary)
                show_summary=true
                shift
                ;;
            --section)
                section_name="$2"
                shift 2
                ;;
            --format)
                output_format="$2"
                shift 2
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
                target_file="$1"
                shift
                ;;
        esac
    done

    # Show header for most operations
    if [[ "$show_list" == false && "$show_summary" == false && -z "$search_query" ]]; then
        print_header
    fi

    # Execute based on arguments
    if [[ "$show_list" == true ]]; then
        list_docs
    elif [[ -n "$search_query" ]]; then
        search_content "$search_query"
    elif [[ "$show_summary" == true ]]; then
        generate_summary
    elif [[ -n "$section_name" ]]; then
        if [[ -z "$target_file" ]]; then
            echo -e "${RED}❌ Error: File is required when specifying a section${NC}" >&2
            exit 1
        fi

        local file_path
        if file_path=$(resolve_file_path "$target_file"); then
            echo -e "${GREEN}📖 Section '${YELLOW}$section_name${GREEN}' from ${CYAN}$target_file${NC}:${NC}"
            echo ""
            get_section_content "$file_path" "$section_name"
        else
            echo -e "${RED}❌ Error: File '$target_file' not found${NC}" >&2
            echo -e "${YELLOW}Available files:${NC}" >&2
            list_docs >&2
            exit 1
        fi
    elif [[ -n "$target_file" ]]; then
        local file_path
        if file_path=$(resolve_file_path "$target_file"); then
            case "$output_format" in
                "summary")
                    format_summary "$file_path"
                    ;;
                *)
                    format_readable "$file_path"
                    ;;
            esac
        else
            echo -e "${RED}❌ Error: File '$target_file' not found${NC}" >&2
            echo -e "${YELLOW}Available files:${NC}" >&2
            list_docs >&2
            exit 1
        fi
    else
        show_help
    fi
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
