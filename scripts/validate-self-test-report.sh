#!/usr/bin/env bash
# validate-self-test-report.sh
# Validates an agent's report.md against the expected report shape for a scenario.

set -euo pipefail

usage() {
    cat <<EOF
Usage: $(basename "$0") <scenario> [report_file]

Validates an agent's report against the expected report shape.

Arguments:
  scenario    Scenario name (e.g., 01-scaffold-cli, 02-scaffold-axum-api)
  report_file  Path to report.md (default: ./report.md)

Examples:
  $(basename "$0") 01-scaffold-cli
  $(basename "$0") 03-refactor-bad-library ./my-project/report.md
EOF
    exit 1
}

SCENARIO="${1:-}"
REPORT_FILE="${2:-./report.md}"

if [[ -z "$SCENARIO" ]]; then
    echo "Error: scenario required"
    usage
fi

# Map scenario to expected report shape file
# Map scenario to expected report shape file
declare -A SHAPE_FILES=(
    ["01-scaffold-cli"]="01-cli-report-shape.md"
    ["02-scaffold-axum-api"]="02-api-report-shape.md"
    ["03-refactor-bad-library"]="03-refactor-report-shape.md"
    ["04-audit-unsafe-ffi"]="04-unsafe-audit-report-shape.md"
    ["05-create-workspace-service"]="05-workspace-report-shape.md"
)

if [[ -z "$SCENARIO" ]]; then
    echo "Error: scenario required"
    usage
fi

shape_filename="${SHAPE_FILES[$SCENARIO]:-}"
if [[ -z "$shape_filename" ]]; then
    echo "Error: unknown scenario '$SCENARIO'"
    echo "Valid scenarios: 01-scaffold-cli 02-scaffold-axum-api 03-refactor-bad-library 04-audit-unsafe-ffi 05-create-workspace-service"
    exit 1
fi

SHAPE_PATH="$PROJECT_ROOT/self-tests/expected-reports/$shape_filename"

if [[ ! -f "$SHAPE_PATH" ]]; then
    echo "Error: expected shape not found at $SHAPE_PATH"
    exit 1
fi

if [[ ! -f "$REPORT_FILE" ]]; then
    echo "Error: report file not found: $REPORT_FILE"
    exit 1
fi

echo "=== Validating Report for Scenario: $SCENARIO ==="
echo "Report: $REPORT_FILE"
echo "Expected shape: $SHAPE_FILE"
echo ""

FAILED=0

# Check required sections
required_sections=("Summary" "Files Changed" "Commands Run" "Risks" "Next Steps" "Final Verdict")
for section in "${required_sections[@]}"; do
    if grep -q "## $section" "$REPORT_FILE"; then
        echo "✓ Found '## $section'"
    else
        echo "✗ Missing '## $section'"
        FAILED=1
    fi
done

echo ""

# Check Files Changed table structure
if grep -q "| File | Change |" "$REPORT_FILE"; then
    echo "✓ Files Changed table has correct columns"
else
    echo "✗ Files Changed table missing or has wrong columns (expected: | File | Change |)"
    FAILED=1
fi

# Check Commands Run table structure
if grep -q "| Command | Result |" "$REPORT_FILE"; then
    echo "✓ Commands Run table has correct columns"
else
    echo "✗ Commands Run table missing or has wrong columns (expected: | Command | Result |)"
    FAILED=1
fi

echo ""

# Check for Final Verdict value
if grep -q "Final Verdict" "$REPORT_FILE"; then
    verdict_line=$(grep -A1 "## Final Verdict" "$REPORT_FILE" | tail -1 | tr -d ' ')
    if echo "$verdict_line" | grep -qiE "^(READEY|READY_WITH_LIMITATIONS|NOT_READY)"; then
        echo "✓ Final Verdict is valid: $verdict_line"
    else
        echo "✗ Final Verdict invalid (expected READY/READY_WITH_LIMITATIONS/NOT_READY): $verdict_line"
        FAILED=1
    fi
fi

echo ""

# Check Commands Run has at least the expected commands (based on scenario)
case "$SCENARIO" in
    01-scaffold-cli|02-scaffold-axum-api)
        for cmd in "cargo build" "cargo test" "cargo clippy"; do
            if grep -q "$cmd" "$REPORT_FILE"; then
                echo "✓ Command '$cmd' appears in report"
            else
                echo "✗ Command '$cmd' missing from report"
                FAILED=1
            fi
        done
        ;;
    03-refactor-bad-library|04-audit-unsafe-ffi|05-create-workspace-service)
        for cmd in "cargo build" "cargo test" "cargo clippy"; do
            if grep -q "$cmd" "$REPORT_FILE"; then
                echo "✓ Command '$cmd' appears in report"
            else
                echo "✗ Command '$cmd' missing from report"
                FAILED=1
            fi
        done
        ;;
esac

echo ""

# Check that commands show PASS/FAIL results
pass_count=$(grep -c "PASS" "$REPORT_FILE" 2>/dev/null || echo "0")
fail_count=$(grep -c "FAIL" "$REPORT_FILE" 2>/dev/null || echo "0")
echo "Summary: $pass_count PASS results, $fail_count FAIL results"

if [[ $fail_count -gt 0 ]]; then
    echo "⚠ Warning: Report contains FAIL results"
fi

echo ""
if [[ $FAILED -eq 0 ]]; then
    echo "=== VALIDATION PASSED ==="
    exit 0
else
    echo "=== VALIDATION FAILED ==="
    exit 1
fi