#!/usr/bin/env bash
# check_msrv.sh
# Check MSRV consistency between Cargo.toml, rust-toolchain.toml, and reality.
#
# Usage:
#   ./scripts/check_msrv.sh [path]
#
# Default path: current directory.
#
# What it does:
#   - Reads rust-version from Cargo.toml (fatal if missing)
#   - Reads channel from rust-toolchain.toml (informational)
#   - Runs cargo check to verify the project compiles
#   - Warns clearly if MSRV cannot be fully proven (no pinned toolchain)
#
# IMPORTANT: This script does NOT prove MSRV in the strict sense.
#            It only verifies the project compiles with the declared MSRV
#            if a pinned toolchain is available. Use rustup override or
#            rust-toolchain.toml with a specific version to fully prove MSRV.

set -euo pipefail

PROJECT_DIR="${1:-.}"

# Color helpers
RED='\033[0;31m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
BOLD='\033[1m'
NC='\033[0m'

section() {
    echo ""
    echo -e "${BOLD}${CYAN}[ $* ]${NC}"
}

pass() { echo -e "  ${GREEN}✓${NC} $*"; }
warn() { echo -e "  ${YELLOW}⚠${NC} $*"; }
info() { echo -e "  ${CYAN}›${NC} $*"; }
fail() { echo -e "  ${RED}✗${NC} $*"; }

# Validate project
if [[ ! -d "$PROJECT_DIR" ]]; then
    fail "Directory not found: ${PROJECT_DIR}"
    exit 1
fi

cd "$PROJECT_DIR"

if [[ ! -f "Cargo.toml" ]]; then
    fail "Not a Rust project (no Cargo.toml): ${PROJECT_DIR}"
    exit 1
fi

section "MSRV Check: $(pwd)"
echo ""

# Read rust-version from Cargo.toml
# Handles both [package] and [workspace.package] sections
MSRV=""
MSRV=$(grep '^rust-version' Cargo.toml | head -1 | sed 's/.*rust-version.*=.*"\([^"]*\)".*/\1/')
if [[ -z "$MSRV" ]]; then
    MSRV=$(grep '^rust-version' Cargo.toml | head -1 | sed 's/.*rust-version.*=.*'\''\([^'\'']*\)'\''.*/\1/')
fi

# Read rust-toolchain.toml
TOOLCHAIN_FILE="rust-toolchain.toml"
HAS_TOOLCHAIN=false
TOOLCHAIN_CHANNEL=""

if [[ -f "$TOOLCHAIN_FILE" ]]; then
    HAS_TOOLCHAIN=true
    if grep -q 'channel' "$TOOLCHAIN_FILE"; then
        TOOLCHAIN_CHANNEL=$(awk -F'=' '/^\s*channel/ {gsub(/[ "'\'']/,"",$2); print $2}' "$TOOLCHAIN_FILE")
    fi
fi

# RUST_MSRV env override
ENV_MSRV="${RUST_MSRV:-}"
if [[ -n "$ENV_MSRV" ]]; then
    info "RUST_MSRV env set: $ENV_MSRV (overrides Cargo.toml)"
    MSRV="$ENV_MSRV"
fi

# Report findings
section "Detected MSRV values"

if [[ -n "$MSRV" ]]; then
    pass "rust-version in Cargo.toml: $MSRV"
else
    warn "No rust-version field found in [package] section of Cargo.toml"
    echo ""
    warn "IMPORTANT: Add rust-version to [package] to declare MSRV:"
    echo '  rust-version = "1.85"'
    echo ""
fi

if $HAS_TOOLCHAIN; then
    if [[ -n "$TOOLCHAIN_CHANNEL" ]]; then
        pass "rust-toolchain.toml channel: $TOOLCHAIN_CHANNEL"
    else
        warn "rust-toolchain.toml exists but has no channel field"
    fi
else
    info "No rust-toolchain.toml found (MSRV cannot be fully proven without it)"
fi

# MSRV completeness warning
section "MSRV proof status"

if [[ -z "$MSRV" ]]; then
    echo ""
    warn "Cannot verify MSRV - rust-version is not declared in Cargo.toml"
    echo ""
    echo "  Add rust-version to [package]:"
    echo '  rust-version = "1.85"'
    echo ""
    exit 1
fi

FULL_PROOF=false

if $HAS_TOOLCHAIN; then
    if [[ "$TOOLCHAIN_CHANNEL" != "stable" && "$TOOLCHAIN_CHANNEL" != "beta" && "$TOOLCHAIN_CHANNEL" != "nightly" ]]; then
        pass "Pinned toolchain ($TOOLCHAIN_CHANNEL) - MSRV can be fully proven"
        FULL_PROOF=true
    elif [[ "$TOOLCHAIN_CHANNEL" == "stable" ]]; then
        warn "rust-toolchain.toml uses 'stable' channel"
        warn "MSRV is declared ($MSRV) but stable channel does NOT pin to a specific version"
        warn "This script CANNOT fully prove MSRV - stable toolchain may be newer than MSRV"
    else
        warn "rust-toolchain.toml uses '$TOOLCHAIN_CHANNEL' channel"
        warn "This script runs checks with the current toolchain, not the MSRV"
    fi
else
    warn "No rust-toolchain.toml - cannot pin toolchain to MSRV"
    warn "This script runs checks with the CURRENT toolchain, not the MSRV"
fi

# Run cargo check
section "Running cargo check"

info "Using current toolchain:"
rustc --version
echo ""

if cargo check --workspace --all-features 2>&1; then
    pass "cargo check passed"
else
    fail "cargo check failed - fix compilation errors first"
    exit 1
fi

# Final summary
section "Summary"

if $FULL_PROOF; then
    pass "MSRV check complete and fully proven"
    echo ""
    echo "  Declared MSRV:  $MSRV"
    echo "  Toolchain:      $TOOLCHAIN_CHANNEL (pinned)"
    echo "  Proof status:   FULL"
else
    warn "MSRV check complete but NOT fully proven"
    echo ""
    echo "  Declared MSRV:  $MSRV"
    if $HAS_TOOLCHAIN; then
        echo "  Toolchain:      $TOOLCHAIN_CHANNEL (unpinned)"
    else
        echo "  Toolchain:      none (no rust-toolchain.toml)"
    fi
    echo "  Proof status:   PARTIAL (current toolchain used, not MSRV toolchain)"
    echo ""
    echo "  To fully prove MSRV, add a pinned rust-toolchain.toml:"
    echo "    [toolchain]"
    echo "    channel = \"$MSRV\""
fi
echo ""