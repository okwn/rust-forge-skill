# Agent Prompt: Unsafe Code Audit

You are an AI coding agent. Before writing any code, **read the rust-forge-skill** skill definition:

```
Read the skill at: rust-forge-skill/SKILL.md
```

---

## Task

Perform a comprehensive unsafe code audit on `{{crate_name}}`.

- **Project type:** FFI wrapper crate with C interop (or WASM module)
- **Focus areas:** Memory safety, unsafe block documentation, FFI boundary security

---

## Steps

1. **Read** the relevant skill guides before writing any code:
   - `rust-forge-skill/guides/07-ffi-c-cpp.md` — FFI patterns and safety
   - `rust-forge-skill/guides/08-security-unsafe-audit.md` — Security audit procedures
   - `rust-forge-skill/guides/12-rust-anti-patterns.md` — Unsafe anti-patterns

2. **Audit all `unsafe` blocks** — produce a table:

   ```bash
   grep -rn "unsafe {" --include="*.rs" src/
   ```

   | File | Line | SAFETY comment? | Invariant stated? | UB consequences documented? |
   |---|---|---|---|---|
   | `src/ffi.rs` | 23 | YES | YES | YES |
   | `src/buffer.rs` | 45 | NO | — | — |

3. **Check FFI boundary:**
   - [ ] All pointers validated before dereference
   - [ ] Memory ownership clear at boundary (who frees what)
   - [ ] `Drop` implementations handle cleanup correctly
   - [ ] `Send`/`Sync` correctly implemented or explicitly forbidden
   - [ ] No data races possible
   - [ ] `CString`/`CStr` conversion errors handled

4. **Run the unsafe audit script:**
   ```bash
   bash rust-forge-skill/scripts/audit_unsafe.sh
   ```
   Fix every issue it reports before proceeding.

5. **Review each unsafe block for security issues:**
   - [ ] No buffer overflows in array/slice access
   - [ ] No integer overflows in size calculations
   - [ ] No use-after-free (check `Box::from_raw`, `c_ptr` usage)
   - [ ] No uninitialized memory reads
   - [ ] Proper error handling at FFI boundary (no panics crossing boundary)

6. **Apply fixes** — document each fix in the report.

---

## Quality Requirements

These are non-negotiable. The deliverable is only accepted when all pass:

- **Every `unsafe` block has a `SAFETY` comment**
- **`SAFETY` comments include:** invariant, why it holds, UB if violated
- **No raw pointer dereferences without validation**
- **No `static mut` declarations** (use atomics or `Mutex`)
- **Memory ownership is explicit at FFI boundary**
- **`unsafe impl` for `Send`/`Sync` is documented**

---

## Validation Commands

Run these commands in sequence. **All must pass.** Report the output of each.

```bash
cargo fmt --all -- --check
echo "=== FORMAT CHECK: PASS ==="

cargo clippy --workspace --all-targets --all-features -- -D warnings -D unsafe_code
echo "=== CLIPPY CHECK: PASS ==="

cargo test --workspace --all-features
echo "=== TEST CHECK: PASS ==="

bash rust-forge-skill/scripts/audit_unsafe.sh
echo "=== UNSAFE AUDIT: PASS ==="
```

---

## Audit Report Template

```markdown
## Unsafe Block Audit Report: {{crate_name}}

### Summary
- Total unsafe blocks: N
- Blocks with SAFETY comments: N
- Blocks without SAFETY comments: N
- Blocks needing fixes: N

### Detailed Findings

#### [file:line] — VERDICT: SAFE / NEEDS FIX

```rust
unsafe {
    // ...
}
```

**SAFETY Review:**
- **Invariant:** ...
- **Why invariant holds:** ...
- **UB if violated:** ...

**Action:** None / Add SAFETY comment / Refactor to safe code

---

### FFI Boundary Review

| Function | Pointer Validation | Ownership | Drop Correct | Notes |
|---|---|---|---|---|
| `exported_func` | YES | Clear | YES | |
| `from_raw_ptr` | PARTIAL | — | — | Missing null check |

---

### Security Issues Found

| Severity | Issue | Location | Fix |
|---|---|---|---|
| HIGH | Buffer overflow possible | `src/buffer.rs:45` | Add bounds check |

---

### Final Verdict

**AUDIT PASS** / **AUDIT FAIL — N issues remain**
```

---

## Deliverables

1. **Audit report** (using the template above) with:
   - All unsafe blocks cataloged with SAFETY documentation reviewed
   - Each block's verdict (SAFE / NEEDS FIX)
   - FFI boundary review table
   - Security issues found with fixes
   - Final verdict

2. **Fixes applied** — all SAFETY comments added, issues resolved

3. **Validation output** — copy the terminal output of each validation command

---

## Anti-Patterns That Fail Audit

- `unsafe` block without `SAFETY` comment
- SAFETY comment that only says `// safe code` without explaining the invariant
- Raw pointer dereference without null check
- `static mut` (always reject — use atomics or `Mutex`)
- FFI boundary where ownership is not documented
- `unsafe impl` without documentation of why `Send`/`Sync` is safe
