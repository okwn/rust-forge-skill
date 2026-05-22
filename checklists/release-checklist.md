# Release Checklist

Use this checklist when releasing a Rust project generated with rust-forge-skill.

---

## Pre-Release

- [ ] All tests pass: `cargo test --workspace --all-features`
- [ ] Formatting applied: `cargo fmt --all`
- [ ] Linting passes: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] Documentation builds: `cargo doc --workspace --all-features --no-deps`
- [ ] MSRV verified: `./scripts/check_msrv.sh` (libraries)
- [ ] Security audit passed: `cargo audit && cargo deny check advisories licenses ban`
- [ ] `CHANGELOG.md` updated with all changes since last release (Keep a Changelog format)
- [ ] Version bumped in `Cargo.toml`:
  - `patch` — bug fixes, no API changes
  - `minor` — new functionality, backward compatible
  - `major` — breaking changes

---

## Semver Review

- [ ] API surface reviewed for breaking changes
- [ ] `Cargo.toml` `version` field updated correctly
- [ ] `lib.rs` public exports reviewed (no new `pub(crate)` or private items exposed)
- [ ] `CHANGELOG.md` lists breaking changes explicitly
- [ ] Migration guide written if breaking changes exist
- [ ] Dependent crates tested against new version

---

## Documentation

- [ ] README.md is current:
  - Build commands are correct
  - Environment variables documented
  - Features flag table up-to-date
  - Example usage works
- [ ] `cargo doc --workspace --all-features --no-deps` generates without warnings
- [ ] `docs.rs` builds correctly (if publishing a library)
- [ ] API docs on `docs.rs` match local docs
- [ ] Migration guide for breaking changes (if any)

---

## Quality Gate — Final

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --release
cargo doc --workspace --all-features --no-deps   # libraries
./scripts/check_msrv.sh                          # libraries
./scripts/audit_unsafe.sh                        # FFI/WASM
```

- [ ] All commands pass before proceeding

---

## Build Artifacts

- [ ] `cargo build --release` succeeds
- [ ] Binary size is reasonable (check with `ls -lh target/release/<binary>`)
- [ ] Release profile settings verified:
  - `lto = "thin"` or `"fat"` enabled
  - `codegen-units = 1` for better optimization
  - `panic = "abort"` set (smaller binaries, no unwinding)
  - `strip = true` for production
- [ ] `cargo check --release --profile=production` validates final build config
- [ ] Cross-compilation targets tested (if applicable)

---

## Publish (Crates.io)

- [ ] `cargo publish --dry-run` succeeds with no warnings
- [ ] Crates.io account authenticated (`cargo login`)
- [ ] Package name available on crates.io
- [ ] `cargo publish` succeeds
- [ ] `Cargo.lock` remains unchanged after publish
- [ ] Tag created: `git tag vX.Y.Z`
- [ ] Tag message matches CHANGELOG entry for this version

---

## CI/CD

- [ ] GitHub Actions workflow passes on `main` branch
- [ ] MSRV check runs in CI
- [ ] Security audit runs in CI
- [ ] Codecov coverage uploaded (if configured)
- [ ] Benchmark baseline recorded (if applicable)

---

## Post-Release

- [ ] Tag pushed: `git push origin vX.Y.Z`
- [ ] GitHub Release created:
  - Title: `vX.Y.Z`
  - Body: Copy from `CHANGELOG.md`
  - Artifacts attached
- [ ] Announced (if applicable: social media, newsletter, changelog feed)
- [ ] Monitoring set up for production (if applicable)
- [ ] Documentation deployed (if applicable)

---

## Rollback Plan

- [ ] Previous version identified
- [ ] `Cargo.toml` reversion steps documented
- [ ] `crates.io` yank command ready: `cargo yank --version X.Y.Z`
- [ ] Deployment rollback steps documented (if applicable)
- [ ] Database migration rollback steps documented (if applicable)

---

## Final Verification

```bash
# Verify the published artifact
cargo install --crate-type bin <crate_name> --version X.Y.Z

# Verify tag
git verify-tag vX.Y.Z

# Verify checksums
sha256sum target/release/<binary>
```

| Step | Status | Notes |
|---|---|---|
| Tests pass | [ ] | |
| Version bumped | [ ] | |
| CHANGELOG updated | [ ] | |
| Semver reviewed | [ ] | |
| Docs current | [ ] | |
| Build succeeds | [ ] | |
| MSRV verified | [ ] | |
| Security audit passed | [ ] | |
| dry-run publish | [ ] | |
| Tag created | [ ] | |
| Tag pushed | [ ] | |
| GitHub Release created | [ ] | |
| Rollback plan documented | [ ] | |
