#!/usr/bin/env bash
# validate_rust_project.sh
# Run a strict validation suite against any Rust project path.
#
# Usage:
#   ./scripts/validate_rust_project.sh [path]
#
# Default path: current directory.

set -euo pipefail

PROJECT_DIR="${1:-.}"

# Color helpers
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

section() {
    echo ""
    echo -e "${BOLD}${CYAN}[ $* ]${NC}"
    echo ""
}

pass() {
    echo -e "  ${GREEN}✓${NC} $*"
}

warn() {
    echo -e "  ${YELLOW}⚠${NC} $*"
}

fail() {
    echo -e "  ${RED}✗${NC} $*"
}

info() {
    echo -e "  ${CYAN}›${NC} $*"
}

# ── Check required commands ──────────────────────────────────────────────────

section "Checking required commands"

check_cmd() {
    if command -v "$1" &>/dev/null; then
        pass "$1 found: $(command -v "$1")"
        return 0
    else
        fail "$1 not found"
        return 1
    fi
}

REQUIRED_CMDS=("cargo" "rustc" "rustfmt")
MISSING=0

for cmd in "${REQUIRED_CMDS[@]}"; do
    check_cmd "$cmd" || ((MISSING++))
done

if ((MISSING > 0)); then
    fail "Missing $MISSING required command(s) — cannot proceed"
    exit 1
fi

cargo --version
rustc --version

# ── Project discovery ───────────────────────────────────────────────────────

section "Discovering project at: ${PROJECT_DIR}"

if [[ ! -d "$PROJECT_DIR" ]]; then
    fail "Directory does not exist: ${PROJECT_DIR}"
    exit 1
fi

cd "$PROJECT_DIR"

if [[ ! -f "Cargo.toml" ]]; then
    fail "No Cargo.toml found — not a Rust project: ${PROJECT_DIR}"
    exit 1
fi

PACKAGE_NAME="$(cargo metadata --no-deps --format-version=1 2>/dev/null | \
    jq -r '.packages[0].name // "unknown"')"

pass "Valid Rust project: ${PACKAGE_NAME}"

# ── Format check ────────────────────────────────────────────────────────────

section "Running cargo fmt"

if cargo fmt --all -- --check 2>&1; then
    pass "Format check passed"
else
    fail "Format check failed — run 'cargo fmt --all' to fix"
    exit 1
fi

# ── Clippy ─────────────────────────────────────────────────────────────────

section "Running cargo clippy"

if cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1; then
    pass "Clippy passed (all warnings as errors)"
else
    fail "Clippy found issues — fix before proceeding"
    exit 1
fi

# ── Tests ──────────────────────────────────────────────────────────────────

section "Running tests"

HAS_NEXTEST=false
if command -v cargo-nextest &>/dev/null; then
    HAS_NEXTEST=true
    info "cargo-nextest detected — using it"
fi

if $HAS_NEXTEST; then
    if cargo nextest run --workspace --all-features 2>&1; then
        pass "Tests passed (cargo nextest)"
    else
        fail "Tests failed"
        exit 1
    fi
else
    if cargo test --workspace --all-features 2>&1; then
        pass "Tests passed (cargo test)"
    else
        fail "Tests failed"
        exit 1
    fi
fi

# ── Security audit ─────────────────────────────────────────────────────────

section "Running security audit"

AUDIT_FOUND=false

if command -v cargo-audit &>/dev/null; then
    info "cargo-audit detected — running"
    if cargo audit 2>&1; then
        pass "Audit passed (no known vulnerabilities)"
    else
        warn "Audit found issues — review output above"
        AUDIT_FOUND=true
    fi
else
    info "cargo-audit not installed — skipped"
fi

if command -v cargo-deny &>/dev/null; then
    info "cargo-deny detected — running"
    if cargo deny check 2>&1; then
        pass "cargo-deny check passed"
    else
        warn "cargo-deny found issues — review output above"
        AUDIT_FOUND=true
    fi
else
    info "cargo-deny not installed — skipped"
fi

# ── Final verdict ──────────────────────────────────────────────────────────

section "Validation complete"

if $AUDIT_FOUND; then
    warn "Audit warnings were found — review above"
fi

pass "All quality gates passed for: ${PACKAGE_NAME}"
echo ""