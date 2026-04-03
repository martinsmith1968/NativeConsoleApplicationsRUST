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

### Decision: hashcalc Module Structure Refactoring

**Date:** Current session  
**Author:** Marcus (Backend Dev)  
**Status:** Implemented ✓  
**Impact:** High — Sets pattern for algorithm-based modularization

#### Problem Statement

The hashcalc binary had all five hash algorithms (SHA1, MD5, SHA256, SHA512, Base64) implemented inline within a single `hash_content()` function in `main.rs`. This led to:
- ~110 lines of algorithm implementation buried in a 1031-line file
- Difficulty isolating and testing individual algorithms
- Unclear separation between CLI orchestration logic and algorithm logic
- Added complexity when reading or modifying the main binary

#### Solution Implemented

Created a dedicated `hashers` module under `src/bin/hashcalc/hashers/` with:
- One file per algorithm (sha1.rs, md5.rs, sha256.rs, sha512.rs, base64.rs)
- Central dispatcher in mod.rs that routes algorithm selection
- Consistent `pub fn hash(data: &[u8]) -> Result<String, String>` signature across all hashers
- Simplified main.rs reduced to ~70 lines of pure CLI logic

#### Key Design Patterns

**1. Uniform Public API**
Each hasher exports the same function signature:
```rust
pub fn hash(data: &[u8]) -> Result<String, String>
```

**2. Module Dispatch Pattern**
The mod.rs file re-exports each hasher's function and provides centralized dispatch:
```rust
pub use self::sha1::hash as sha1;
pub use self::md5::hash as md5;
// ... etc

pub fn hash_content(content_bytes: &[u8], algorithm: &str) -> Result<String, String> {
    match algorithm {
        "sha1" => self::sha1(content_bytes),
        "md5" => self::md5(content_bytes),
        // ... etc
    }
}
```

**3. Trait Import Handling**
Each hasher that uses external crate traits imports them locally, no cross-module conflicts despite sha1 and sha2 crates depending on different digest versions.

#### Verification

✅ All 67 integration tests pass without modification  
✅ Backward compatibility: CLI interface unchanged  
✅ Build: Clean, zero warnings  
✅ Manual testing: Verified SHA256, SHA1, Base64 produce correct output  

#### Rationale

- **Maintainability**: Each algorithm is now a focused, independently testable unit
- **Clarity**: Readers can understand one algorithm without context switching
- **Extensibility**: New algorithms don't clutter the main file
- **Team alignment**: Establishes reusable pattern for multi-algorithm CLI tools across the project

---

### Decision: Modular Hasher Architecture for hashcalc

**Date:** Current session  
**Author:** Marcus (Backend Dev), Reviewed by Kiefer (Lead)  
**Status:** Approved ✓  
**Impact:** High — Core pattern for multi-algorithm CLI tools

#### Summary

Refactored hashcalc from monolithic single-file structure into modular architecture with dedicated hasher modules under `src/bin/hashcalc/hashers/`.

#### Architecture

```
hashers/
├── mod.rs          # Central dispatcher, hash_content(bytes, algorithm) -> Result
├── sha1.rs         # Isolated sha1 implementation
├── md5.rs          # Isolated md5 implementation
├── sha256.rs       # Isolated sha256 implementation
├── sha512.rs       # Isolated sha512 implementation
└── base64.rs       # Isolated base64 implementation

main.rs             # Orchestration, file I/O, CLI parsing (~70 lines, was ~1031)
```

#### Key Patterns

1. **Uniform Function Signatures:** All hashers expose `pub fn hash(data: &[u8]) -> Result<String, String>`
2. **Central Dispatcher:** Single `hash_content()` function routes algorithm selection—no duplicate logic
3. **Error Handling:** Result types propagated throughout, no panics or `.unwrap()` calls
4. **Trait Adaptation:** Different crate interfaces handled correctly (sha1/sha2 use Digest trait, md5 uses direct function)

#### Rationale

- **Maintainability:** Each algorithm isolated and testable independently
- **Extensibility:** Adding a new algorithm requires minimal changes (new file, add match arm)
- **Clarity:** Main logic simplified from ~1031 to ~70 lines
- **Backward Compatibility:** CLI interface unchanged, all 67 tests pass unmodified

#### Verification

- ✅ Build: Clean, zero warnings
- ✅ Tests: 67/67 pass (debug and release)
- ✅ Spot checks: SHA256, MD5, Base64 verified against known test vectors
- ✅ Error handling: File not found, invalid algorithm, mutual exclusivity enforced

#### Approval

**Kiefer (Lead):** Code is idiomatic Rust, architecture sound, tests comprehensive. Approved for merge.

---

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
