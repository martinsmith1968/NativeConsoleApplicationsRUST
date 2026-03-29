# Squad Decisions

## Active Decisions

### Decision: Comprehensive hashcalc Test Suite

**Date:** Current session  
**Author:** Blake (Tester)  
**Status:** Complete ✓

Expanded hashcalc test suite from 40 to 67 comprehensive tests covering:
- All five algorithms (SHA1, MD5, SHA256, SHA512, Base64)
- Text and file modes with all algorithms
- Extensive edge cases (unicode, binary, large data, empty, special chars)
- Error scenarios with exit code validation
- Consistency and equivalence verification
- Output format validation

**Rationale:** CLI tools require extensive testing across edge cases and error scenarios to catch regressions and ensure robust error handling. 67-test suite provides high confidence that all algorithm combinations work correctly.

**Verification:** ✓ All 67 tests pass in debug mode, ✓ All 67 tests pass in release mode, ✓ No compiler warnings, ✓ File/text equivalence verified

---

### Decision: hashcalc Test Suite Architecture

**Date:** Current session  
**Author:** Blake (Tester)  
**Status:** Implemented ✓  
**Impact:** High - Core testing patterns for CLI hashing tools

Built comprehensive test suite using mixed unit (individual functions) + integration (full CLI flow) approaches to ensure:
- Text mode hashing works correctly
- File mode reads and hashes files correctly
- Mutual exclusivity between text/--file enforced
- Error handling for missing files and bad arguments
- Exit codes match expected behavior
- Output format consistent across runs

**Key Patterns:**
1. Unit tests for `hash_content()` and `read_file_contents()` functions
2. CLI tests spawning `cargo run` for real-world validation
3. Known hash validation against standard test vectors
4. Temporary file handling with `test_` prefix and deterministic cleanup
5. Exit code testing for success (0) and error (1) cases
6. Output format validation for consistency

---

### Decision: Comprehensive Integration Tests for uuidgen

**Date:** Current Session  
**Status:** Implemented  
**Affected Component:** `src/bin/uuidgen/main.rs`

Created comprehensive integration test suite (46 tests) within the binary using inline `#[cfg(test)]` module to validate:
- V6 seed parsing (bytes apply correctly)
- Template error handling (no panics)
- Count type upgrade (u32 support)
- Cloning overhead (redundant clones removed)
- Control flow refactoring (match expressions)
- Function signatures (&str acceptance)

**Rationale:** Inline tests can directly access internal functions and verify end-to-end behavior. Better visibility into private function behavior and easier maintenance alongside code changes.

**Results:** ✅ All 46 tests passing, ✅ All 6 bug fixes have direct test coverage, ✅ No breaking changes

---

### Decision: Graceful Template Error Handling

**Date:** 2024  
**Author:** Marcus (Backend Dev)  
**Status:** Implemented

The uuidgen `format_output()` function used `.unwrap()` on `strfmt()` result, causing panics when users provided malformed output templates (e.g., `{invalid}`).

**Solution:** Replace `.unwrap()` with `match` on the `Result` type, logging errors to stderr and falling back to returning the unformatted UUID.

**Rationale:** CLI tools should degrade gracefully rather than crash. Malformed templates are user input errors, not programming errors, so they should be handled at runtime with meaningful feedback.

**Impact:** Fixes crash on invalid template syntax, improves CLI robustness, no breaking changes

---

### Decision: Handling Multi-Version Digest Trait Imports

**Date:** Session 3  
**Author:** Marcus  
**Status:** Implemented

The hashcalc multi-algorithm support required importing the `Digest` trait from both sha2 (0.11.0-rc.5) and sha1 (0.10.6) crates. These crates depend on different versions of the digest crate, which creates conflicts.

**Solution:** Import each crate's `Digest` trait with explicit module aliases and use direct type method calls rather than trait objects.

```rust
use sha2::{Sha256, Sha512};
use sha1::Sha1;
```

Both `sha1::Sha1` and `sha2::{Sha256, Sha512}` are directly usable without qualification because the `.update()` and `.finalize()` methods are impl'd on each type.

**Rationale:** When multiple crates depend on different versions of a transitive dependency, prefer direct type method calls over trait objects. The Rust trait system ensures the correct impl blocks are used at compile time.

---

### Directive: Never Auto-Commit Changes

**Date:** 2026-03-29T19:36:35Z  
**By:** Martin Smith (via Copilot)

All changes must be code reviewed before committing. No agent should auto-commit changes without explicit approval.

**Why:** Establishes mandatory review gate before any commits  
**Impact:** All agents must stage changes and wait for reviewer approval before running git commit

---

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
