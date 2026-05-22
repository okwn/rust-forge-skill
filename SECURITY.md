# Security Policy — rust-forge-skill

This document describes the security policy for the `rust-forge-skill` pack itself — not the security of projects built using it.

---

## Reporting Security Issues

If you find a security issue in `rust-forge-skill` itself (e.g., a script that exposes secrets, a template that generates insecure code), **do not open a public issue**.

Please report vulnerabilities via one of:

| Method | Contact |
|---|---|
| Email | Open an issue requesting contact details, then send privately |
| GitHub Security Advisories | Use [GitHub's private vulnerability reporting](https://github.com/your-org/rust-forge-skill/security/advisories/new) |

Reports are acknowledged within 48 hours. We aim to respond within 7 days with a fix or a timeline.

---

## Scope

`rust-forge-skill` is a skill pack — a collection of templates, scripts, and guides. Its own security surface is small:

| Component | Security Surface |
|---|---|
| `scripts/*.sh` | Shell injection risk if paths are not sanitized |
| `templates/` | Template code that agents copy into projects |
| `ci/` | CI workflows that may execute user-provided code |
| `mcp/` | Manifest parsing by agent tools |

Projects **generated using** this skill are outside the scope of this policy — they are the responsibility of their authors.

---

## Security Rules for this Skill Pack

### Scripts Must Not Expose Secrets

- No secrets (API keys, tokens, passwords) in shell scripts
- `tracing` commands in scripts must not echo env vars containing secrets
- Scripts must not write sensitive data to stdout in non-error cases

### Templates Must Not Generate Insecure Code by Default

Templates must not generate code that:
- Disables Rust's safety checks (`#[allow(unsafe_code)]` is forbidden in default templates)
- Uses hardcoded credentials or secrets
- Disables certificate verification
- Uses deprecated cryptographic primitives

### CI Workflows Must Not Leak Secrets

- CI workflows must use GitHub secrets, not inline secrets
- Workflows must not echo secrets in logs (`set -x` is forbidden in security workflow)

---

## Unsafe Code Policy

`rust-forge-skill` templates and scripts aim to minimize `unsafe` usage.

### Rules

1. **No `unsafe` in scripts** — shell scripts must not use `unsafe` concepts (not applicable)
2. **Templates** — `unsafe` blocks in templates must have `SAFETY` comments. Templates should prefer safe Rust.
3. **FFI template** — the `ffi-wrapper` template inherently requires `unsafe`; SAFETY comments are mandatory and enforced by the `audit_unsafe.sh` script
4. **WASM template** — minimal `unsafe` usage acceptable for low-level operations; SAFETY comments required

### SAFETY Comment Format

Every `unsafe` block in a template must have a comment in this format:

```rust
// SAFETY: [Invariant]
// Why the invariant holds: [explanation]
// UB if violated: [consequence]
unsafe {
    // ...
}
```

The `audit_unsafe.sh` script verifies this format. Blocks without conforming comments cause the script to exit with code 1.

---

## Dependencies

- `rust-forge-skill` itself has no runtime dependencies
- `cargo audit` and `cargo deny` are used in CI to check for vulnerabilities in dependencies of projects generated using the skill
- The skill pack does not pin to specific crate versions (it generates projects that manage their own dependencies)
- The `deny.toml` file provides a starting configuration — consuming projects should customise it

---

## Security Workflow

The `ci/github-actions-rust-security.yml` workflow runs these checks on every push:

| Check | Tool | Failure Means |
|---|---|---|
| Vulnerability audit | `cargo audit` | Known CVE in dependency tree → FAIL |
| License/banned crate check | `cargo deny` | Prohibited license or banned crate → FAIL |
| Unsafe code audit | `audit_unsafe.sh` | `unsafe` without conforming SAFETY comment → FAIL |
| Unsafe clippy | `cargo clippy -D unsafe_code` | `unsafe_code` lint triggered → FAIL |
| Dependency size audit | `cargo metadata` | Unexpectedly large dependency tree → WARN |

This workflow runs in addition to the standard CI pipeline.

---

## Security Checklist Before Release

Before each `rust-forge-skill` release:

- [ ] `audit_unsafe.sh` passes on all templates
- [ ] No new `unsafe` blocks added to templates without SAFETY comments
- [ ] CI security workflow passes on `main`
- [ ] No secrets in shell scripts (run `grep -rn "secret\|token\|password\|api_key" scripts/`)
- [ ] Template READMEs do not suggest disabling security checks
- [ ] `deny.toml` license list reviewed for completeness
