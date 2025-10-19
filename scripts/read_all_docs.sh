#!/usr/bin/env bash
# read_all_docs.sh - Script to read all documentation and collect errors/warnings/TODOs

set -euo pipefail

# Configuration
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$REPO_ROOT/.docs_reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="$REPORT_DIR/docs_analysis_$TIMESTAMP.txt"
SUMMARY_FILE="$REPO_ROOT/docs_summary.txt"

# Create reports directory
mkdir -p "$REPORT_DIR"

echo "=== Documentation Analysis Started at $(date) ===" | tee "$REPORT_FILE"
echo "Repository: $REPO_ROOT" | tee -a "$REPORT_FILE"

# Step 1: Find all documentation files
echo -e "\n## Step 1: Finding Documentation Files ##" | tee -a "$REPORT_FILE"
find "$REPO_ROOT" -type f \( -name "*.md" -o -name "README*" -o -name "*.txt" \) -not -path "*/.git/*" -not -path "*/target/*" | sort | tee -a "$REPORT_FILE"

# Step 2: Extract all TODO/FIXME/NOTE items
echo -e "\n## Step 2: TODO/FIXME/NOTE Items ##" | tee -a "$REPORT_FILE"
echo "Searching for TODO items..." | tee -a "$REPORT_FILE"
grep -rIn --include="*.md" --include="*.rs" --include="*.sh" --include="*.toml" --exclude-dir=.git --exclude-dir=target "TODO" "$REPO_ROOT" || echo "No TODO items found" | tee -a "$REPORT_FILE"

echo -e "\nSearching for FIXME items..." | tee -a "$REPORT_FILE"
grep -rIn --include="*.md" --include="*.rs" --include="*.sh" --include="*.toml" --exclude-dir=.git --exclude-dir=target "FIXME" "$REPO_ROOT" || echo "No FIXME items found" | tee -a "$REPORT_FILE"

echo -e "\nSearching for NOTE items..." | tee -a "$REPORT_FILE"
grep -rIn --include="*.md" --include="*.rs" --include="*.sh" --include="*.toml" --exclude-dir=.git --exclude-dir=target "NOTE" "$REPO_ROOT" || echo "No NOTE items found" | tee -a "$REPORT_FILE"

# Step 3: Analyze project structure
echo -e "\n## Step 3: Project Structure Analysis ##" | tee -a "$REPORT_FILE"
echo "Directory structure:" | tee -a "$REPORT_FILE"
tree "$REPO_ROOT" -I '.git|target' || find "$REPO_ROOT" -type d -not -path "*/.git/*" -not -path "*/target/*" | sort | tee -a "$REPORT_FILE"

# Step 4: Check for existing configuration files
echo -e "\n## Step 4: Configuration Files ##" | tee -a "$REPORT_FILE"
for config in Cargo.toml package.json Dioxus.toml agent.md ROADMAP.md README.md; do
    if [ -f "$REPO_ROOT/$config" ]; then
        echo "Found: $config" | tee -a "$REPORT_FILE"
        echo "--- Content preview ---" | tee -a "$REPORT_FILE"
        head -20 "$REPO_ROOT/$config" | tee -a "$REPORT_FILE"
        echo "--- End preview ---" | tee -a "$REPORT_FILE"
    else
        echo "Missing: $config" | tee -a "$REPORT_FILE"
    fi
done

# Step 5: Look for any existing code files
echo -e "\n## Step 5: Code Files Analysis ##" | tee -a "$REPORT_FILE"
echo "Rust files:" | tee -a "$REPORT_FILE"
find "$REPO_ROOT" -name "*.rs" -not -path "*/target/*" | tee -a "$REPORT_FILE"

echo -e "\nShell scripts:" | tee -a "$REPORT_FILE"
find "$REPO_ROOT" -name "*.sh" -not -path "*/.git/*" | tee -a "$REPORT_FILE"

# Step 6: Error and Warning Indicators
echo -e "\n## Step 6: Potential Issues ##" | tee -a "$REPORT_FILE"
echo "Searching for error indicators in docs..." | tee -a "$REPORT_FILE"
grep -rIn --include="*.md" -i "error\|warning\|issue\|problem\|bug" "$REPO_ROOT/docs" || echo "No error indicators found in docs" | tee -a "$REPORT_FILE"

# Step 7: Requirements Extraction
echo -e "\n## Step 7: Requirements from Documentation ##" | tee -a "$REPORT_FILE"
echo "Extracting requirements statements..." | tee -a "$REPORT_FILE"
grep -rIn --include="*.md" -i "must\|should\|require\|need\|implement\|feature" "$REPO_ROOT/docs" || echo "No explicit requirements found" | tee -a "$REPORT_FILE"

# Create summary
echo -e "\n## Summary ##" | tee -a "$REPORT_FILE"
TODO_COUNT=$(grep -rI "TODO" "$REPO_ROOT" --include="*.md" --include="*.rs" --exclude-dir=.git --exclude-dir=target | wc -l || echo "0")
FIXME_COUNT=$(grep -rI "FIXME" "$REPO_ROOT" --include="*.md" --include="*.rs" --exclude-dir=.git --exclude-dir=target | wc -l || echo "0")
MD_FILES=$(find "$REPO_ROOT" -name "*.md" -not -path "*/.git/*" | wc -l)
RS_FILES=$(find "$REPO_ROOT" -name "*.rs" -not -path "*/target/*" | wc -l)

echo "Documentation Statistics:" | tee -a "$REPORT_FILE"
echo "- Markdown files: $MD_FILES" | tee -a "$REPORT_FILE"
echo "- Rust files: $RS_FILES" | tee -a "$REPORT_FILE"
echo "- TODO items: $TODO_COUNT" | tee -a "$REPORT_FILE"
echo "- FIXME items: $FIXME_COUNT" | tee -a "$REPORT_FILE"

# Copy summary to main summary file
cp "$REPORT_FILE" "$SUMMARY_FILE"

echo -e "\n=== Documentation Analysis Completed at $(date) ===" | tee -a "$REPORT_FILE"
echo "Full report saved to: $REPORT_FILE"
echo "Summary saved to: $SUMMARY_FILE"

# Display summary on screen
echo -e "\n=== QUICK SUMMARY ==="
cat "$SUMMARY_FILE" | grep -E "(TODO|FIXME|Markdown files|Rust files|Found:|Missing:)" || true
