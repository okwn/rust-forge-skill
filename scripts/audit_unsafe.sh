#!/usr/bin/env bash
# audit_unsafe.sh
# Find unsafe usage and produce an audit checklist.
#
# Usage:
#   ./scripts/audit_unsafe.sh [path]
#
# Default path: current directory.
# Creates unsafe-audit-report.md in the target directory.

set -euo pipefail

PROJECT_DIR="${1:-.}"

# ── Helpers ────────────────────────────────────────────────────────────────

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

section() {
    echo ""
    echo -e "${BOLD}${CYAN}[ $* ]${NC}"
}

pass() { echo -e "  ${GREEN}✓${NC} $*"; }
warn() { echo -e "  ${YELLOW}⚠${NC} $*"; }
info() { echo -e "  ${CYAN}›${NC} $*"; }

# ── Validate ───────────────────────────────────────────────────────────────

if [[ ! -d "$PROJECT_DIR" ]]; then
    echo "Error: directory not found: ${PROJECT_DIR}"
    exit 1
fi

cd "$PROJECT_DIR"

if [[ ! -f "Cargo.toml" ]]; then
    echo "Error: not a Rust project (no Cargo.toml): ${PROJECT_DIR}"
    exit 1
fi

section "Unsafe Audit for: $(pwd)"
echo ""

# ── Patterns to search ────────────────────────────────────────────────────
# Each entry: label | grep pattern | files to exclude

declare -a SEARCH_PATTERNS=(
    "unsafe block|unsafe\s*\{"
    "extern \"C\"|extern\s+\"C\""
    "no_mangle attribute|#\[no_mangle\]"
    "repr(C)|#\[repr\(C\)\]"
    "transmute|transmute"
    "from_raw|from_raw"
    "into_raw|into_raw"
    "get_unchecked|get_unchecked"
    "unwrap_unchecked|unwrap_unchecked"
)

declare -a EXCLUDE_DIRS=("target" ".git" "examples" "tests" "benches")

exclude_arg() {
    local result=""
    for d in "${EXCLUDE_DIRS[@]}"; do
        result+=" --exclude-dir=$d"
    done
    echo "$result"
}

grep_rust() {
    local pattern="$1"
    # Only search .rs files outside target/, .git/, examples/, tests/, benches/
    grep -rn --include="*.rs" \
        --exclude-dir=target \
        --exclude-dir=.git \
        --exclude-dir=examples \
        --exclude-dir=tests \
        --exclude-dir=benches \
        -E "$pattern" . 2>/dev/null || true
}

# ── Collect findings ──────────────────────────────────────────────────────

declare -A findings
declare -A findings_count

for entry in "${SEARCH_PATTERNS[@]}"; do
    IFS='|' read -r label pattern <<< "$entry"
    count=$(grep_rust "$pattern" | wc -l)
    findings["$label"]="$(grep_rust "$pattern")"
    findings_count["$label"]="$count"
done

# Also check for unsafe without SAFETY comment
unsafe_no_safety=$(grep_rust 'unsafe\s*\{' | while IFS=: read -r file line _; do
    # Check if SAFETY comment appears within 3 lines before or after
    context=$(sed -n "$((line>3?line-3:1)),$((line+3))p" "$file")
    if ! echo "$context" | grep -qi "SAFETY"; then
        echo "$file:$line"
    fi
done || true)
unsafe_no_safety_count=$(echo "$unsafe_no_safety" | grep -c . || true)

# static mut
static_mut=$(grep_rust 'static\s+mut')
static_mut_count=$(echo "$static_mut" | grep -c . || true)

# unwrap in non-test/bench/example code
unwrap_prod=$(grep_rust '\.(unwrap|expect)\(' | grep -v "examples/" | grep -v "/tests/" | grep -v "/benches/" || true)
unwrap_prod_count=$(echo "$unwrap_prod" | grep -c . || true)

# println/eprintln in production
println_prod=$(grep_rust 'println!|eprintln!' | grep -v "examples/" | grep -v "/tests/" | grep -v "/benches/" || true)
println_prod_count=$(echo "$println_prod" | grep -c . || true)

# ── Print summary ─────────────────────────────────────────────────────────

section "Pattern inventory"

total=0
for label in "${!findings_count[@]}"; do
    count=${findings_count[$label]}
    if ((count > 0)); then
        warn "$label: $count occurrence(s)"
        ((total += count))
    else
        pass "$label: none"
    fi
done

if ((unsafe_no_safety_count > 0)); then
    warn "unsafe blocks WITHOUT SAFETY comment: $unsafe_no_safety_count"
    ((total += unsafe_no_safety_count))
else
    pass "unsafe blocks WITHOUT SAFETY comment: none"
fi

if ((static_mut_count > 0)); then
    warn "static mut declarations: $static_mut_count"
    ((total += static_mut_count))
else
    pass "static mut declarations: none"
fi

if ((unwrap_prod_count > 0)); then
    warn ".unwrap()/.expect() in production: $unwrap_prod_count"
fi

if ((println_prod_count > 0)); then
    warn "println!/eprintln! in production: $println_prod_count"
fi

echo ""
info "Total unsafe-related findings: $total"

# ── Generate report ───────────────────────────────────────────────────────

REPORT_FILE="unsafe-audit-report.md"

cat > "$REPORT_FILE" << HEADER
# Unsafe Audit Report

**Project:** $(pwd)
**Date:** $(date -u +"%Y-%m-%dT%H:%M:%SZ")
**Branch:** $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "N/A")

---

## 1. Unsafe Inventory

| Pattern | Count |
|---------|-------|
HEADER

for label in "${!findings_count[@]}"; do
    count=${findings_count[$label]}
    echo "| \`$label\` | $count |" >> "$REPORT_FILE"
done

cat >> "$REPORT_FILE" << 'SECTION2'

---

## 2. Unsafe Blocks — Missing SAFETY Comments

SECTION2

if ((unsafe_no_safety_count > 0)); then
    echo '```' >> "$REPORT_FILE"
    echo "$unsafe_no_safety" >> "$REPORT_FILE"
    echo '```' >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "**Action required:** Every \`unsafe {}\` block must have a \`// SAFETY:\` comment explaining:" >> "$REPORT_FILE"
    echo "1. What invariant must be maintained" >> "$REPORT_FILE"
    echo "2. Why this invariant holds at this call site" >> "$REPORT_FILE"
    echo "3. What undefined behavior occurs if the invariant is violated" >> "$REPORT_FILE"
else
    echo "None found. ✓" >> "$REPORT_FILE"
fi

cat >> "$REPORT_FILE" << 'SECTION3'

---

## 3. FFI Boundary Checklist

> For each `extern "C"` block, verify:

- [ ] All FFI signatures are necessary (no over-exposing)
- [ ] Memory safety is documented for pointer parameters
- [ ] Caller/callee ownership semantics are documented
- [ ] `#[repr(C)]` structs have documented field ABI guarantees
- [ ] `transmute` usages have documented type layout assumptions

SECTION3

# Static mut section
cat >> "$REPORT_FILE" << 'SECTION4'

---

## 4. Static Mut Declarations

SECTION4

if ((static_mut_count > 0)); then
    echo '```' >> "$REPORT_FILE"
    echo "$static_mut" >> "$REPORT_FILE"
    echo '```' >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "**REQUIRED:** Replace \`static mut\` with \`Mutex\<T\>\` or \`AtomicT\` from \`std::sync\`." >> "$REPORT_FILE"
else
    echo "None found. ✓" >> "$REPORT_FILE"
fi

# unwrap in production
cat >> "$REPORT_FILE" << 'SECTION5'

---

## 5. .unwrap() / .expect() in Production Code

Found: $unwrap_prod_count occurrence(s) (excluding tests/benches/examples)

SECTION5

if ((unwrap_prod_count > 0)); then
    echo '```' >> "$REPORT_FILE"
    echo "$unwrap_prod" | head -20 >> "$REPORT_FILE"
    echo '```' >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "**Recommendation:** Replace with proper error propagation (\`?\`) or \`.unwrap_or()\` / \`.unwrap_or_else()\` with safe fallbacks." >> "$REPORT_FILE"
else
    echo "None found in production paths. ✓" >> "$REPORT_FILE"
fi

# println in production
cat >> "$REPORT_FILE" << 'SECTION6'

---

## 6. println! / eprintln! in Production Code

Found: $println_prod_count occurrence(s) (excluding tests/benches/examples)

SECTION6

if ((println_prod_count > 0)); then
    echo '```' >> "$REPORT_FILE"
    echo "$println_prod" | head -20 >> "$REPORT_FILE"
    echo '```' >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "**REQUIRED:** Replace \`println!\` / \`eprintln!\` with \`tracing::info!\` / \`tracing::error!\`." >> "$REPORT_FILE"
else
    echo "None found in production paths. ✓" >> "$REPORT_FILE"
fi

# Detailed unsafe block listing
cat >> "$REPORT_FILE" << 'SECTION7'

---

## 7. Detailed Unsafe Block Locations

SECTION7

if ((total > 0)); then
    for entry in "${SEARCH_PATTERNS[@]}"; do
        IFS='|' read -r label pattern <<< "$entry"
        count=${findings_count[$label]}
        if ((count > 0)); then
            echo "### \`$label\` ($count occurrences)" >> "$REPORT_FILE"
            echo '```' >> "$REPORT_FILE"
            echo "${findings[$label]}" | head -30 >> "$REPORT_FILE"
            echo '```' >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"
        fi
    done
else
    echo "No unsafe patterns found." >> "$REPORT_FILE"
fi

# Sign-off section
cat >> "$REPORT_FILE" << 'EOF'

---

## 8. Reviewer Sign-Off

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Author | | | |
| Reviewer | | | |
| Security SME | | | |

**Notes:**

-

EOF

echo ""
info "Report written to: ${REPORT_FILE}"
echo ""

# ── Exit code ───────────────────────────────────────────────────────────────

if ((total > 0)) || ((static_mut_count > 0)) || ((unsafe_no_safety_count > 0)); then
    echo "⚠  Audit complete — $total unsafe-related issue(s) found"
    echo "   Review ${REPORT_FILE} for the full checklist"
    exit 1
else
    echo "✓  Audit complete — no unsafe issues found"
    exit 0
fi