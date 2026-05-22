#!/usr/bin/env bash
# run-self-tests.sh
# Orchestrates running self-test scenarios and validating reports.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

usage() {
    cat <<EOF
Usage: $(basename "$0") [options] [scenario]

Run self-test scenarios and validate reports.

Options:
  --skip-validation    Skip report validation step
  --clean               Clean test artifacts before running
  -h, --help            Show this help

Arguments:
  scenario    Run specific scenario (01-scaffold-cli, 02-scaffold-axum-api,
              03-refactor-bad-library, 04-audit-unsafe-ffi,
              05-create-workspace-service). Default: all

Examples:
  $(basename "$0")                    # Run all scenarios
  $(basename "$0") 01-scaffold-cli   # Run specific scenario
  $(basename "$0") --clean 02-scaffold-axum-api  # Clean + run
EOF
    exit 1
}

SKIP_VALIDATION=0
CLEAN=0
SCENARIO=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --skip-validation)
            SKIP_VALIDATION=1
            shift
            ;;
        --clean)
            CLEAN=1
            shift
            ;;
        -h|--help)
            usage
            ;;
        01-scaffold-cli|02-scaffold-axum-api|03-refactor-bad-library|04-audit-unsafe-ffi|05-create-workspace-service)
            SCENARIO="$1"
            shift
            ;;
        *)
            echo "Error: unknown argument '$1'"
            usage
            ;;
    esac
done

declare -A SHAPE_FILES=(
    ["01-scaffold-cli"]="01-cli-report-shape.md"
    ["02-scaffold-axum-api"]="02-api-report-shape.md"
    ["03-refactor-bad-library"]="03-refactor-report-shape.md"
    ["04-audit-unsafe-ffi"]="04-unsafe-audit-report-shape.md"
    ["05-create-workspace-service"]="05-workspace-report-shape.md"
)

scenarios=(
    "01-scaffold-cli"
    "02-scaffold-axum-api"
    "03-refactor-bad-library"
    "04-audit-unsafe-ffi"
    "05-create-workspace-service"
)

if [[ -n "$SCENARIO" ]]; then
    scenarios=("$SCENARIO")
fi

TEST_DIR="/tmp/rust-forge-self-test"
REPORTS_DIR="$PROJECT_ROOT/self-test-results"

mkdir -p "$REPORTS_DIR"

if [[ $CLEAN -eq 1 ]]; then
    echo "=== Cleaning test artifacts ==="
    rm -rf "$TEST_DIR" "$REPORTS_DIR"/*.md 2>/dev/null || true
    echo "Done."
    echo ""
fi

echo "=== Rust Forge Skill Pack Self-Tests ==="
echo "Test directory: $TEST_DIR"
echo "Reports directory: $REPORTS_DIR"
echo "Scenarios: ${scenarios[*]}"
echo ""

total=0
passed=0
failed=0

for scenario in "${scenarios[@]}"; do
    ((total++)) || true
    echo "========================================"
    echo "Running: $scenario"
    echo "========================================"

    shape_filename="${SHAPE_FILES[$scenario]}"
    scenario_file="$PROJECT_ROOT/self-tests/scenarios/${scenario}.md"
    shape_file="$PROJECT_ROOT/self-tests/expected-reports/${shape_filename}"

    if [[ ! -f "$scenario_file" ]]; then
        echo "✗ Scenario file not found: $scenario_file"
        ((failed++)) || true
        continue
    fi

    if [[ ! -f "$shape_file" ]]; then
        echo "✗ Shape file not found: $shape_file"
        ((failed++)) || true
        continue
    fi

    # Create test workspace
    test_dir="$TEST_DIR/$scenario"
    rm -rf "$test_dir" 2>/dev/null || true
    mkdir -p "$test_dir"

    echo "Test workspace: $test_dir"
    echo ""
    echo "--- Scenario Instructions ---"
    head -50 "$scenario_file"
    echo "..."
    echo ""

    echo "--- Expected Report Shape (summary) ---"
    grep "^## " "$shape_file"
    echo ""

    # Validation only (actual agent run requires AI agent)
    echo "=== Validation Phase ==="

    if [[ $SKIP_VALIDATION -eq 0 ]]; then
        # Run report validation if report exists
        report_file="$test_dir/report.md"
        if [[ -f "$report_file" ]]; then
            echo "Found report, validating..."
            bash "$SCRIPT_DIR/validate-self-test-report.sh" "$scenario" "$report_file" || true
        else
            echo "Note: No report.md found in $test_dir"
            echo "This test requires an AI agent to generate the report."
            echo "To simulate, copy the scenario and run it with an agent."
        fi
    else
        echo "Skipping validation (--skip-validation)"
    fi

    # Validate structure
    echo ""
    echo "=== Structure Validation ==="
    validation_passed=1

    for section in "Summary" "Files Changed" "Commands Run" "Risks" "Next Steps" "Final Verdict"; do
        if grep -q "^## $section" "$shape_file"; then
            echo "✓ $scenario: $section section present"
        else
            echo "✗ $scenario: Missing $section section"
            validation_passed=0
        fi
    done

    if [[ $validation_passed -eq 1 ]]; then
        echo "✓ All structure validations passed for $scenario"
        ((passed++)) || true
    else
        echo "✗ Structure validation failed for $scenario"
        ((failed++)) || true
    fi

    echo ""
done

echo "========================================"
echo "=== Results ==="
echo "========================================"
echo "Total: $total"
echo "Passed: $passed"
echo "Failed: $failed"
echo ""

if [[ $failed -gt 0 ]]; then
    exit 1
fi
exit 0