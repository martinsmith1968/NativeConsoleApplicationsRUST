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

### Decision: Dual Test Strategy for uuidgen (Unit + Integration)

**Date:** Current session  
**Author:** Blake (Tester)  
**Status:** Implemented ✓  
**Impact:** High - Establishes comprehensive testing pattern for CLI applications

#### Problem Statement

The uuidgen binary had 46 inline unit tests covering internal functions but lacked:
- Black-box CLI testing (argument parsing, exit codes, help output)
- Edge case validation (overflow values, malformed inputs, boundary conditions)
- Real-world usage scenarios (combining multiple flags, template rendering through CLI)

#### Solution Implemented

Implemented dual test strategy:

**1. Unit Tests (54 tests in `uuidgen/src/main.rs`)**
- Test internal functions directly
- Added 8 new edge case tests:
  - V6 seed parsing with overflow, negative, extra commas
  - Whitespace-only templates
  - Full pipeline integration (uppercase + non-hyphenated + template)

**2. Integration Tests (30 tests in `uuidgen/tests/integration_tests.rs`)**
- Black-box CLI testing via `assert_cmd` crate
- Spawn actual binary with arguments
- Validate stdout/stderr, exit codes, output format
- Test scenarios:
  - Help and version flags
  - All UUID types and formatting options
  - Template rendering
  - Invalid arguments and error handling
  - Short and long option variants

#### Key Patterns

**CLI Testing with assert_cmd:**
```rust
let mut cmd = Command::cargo_bin("uuidgen").unwrap();
cmd.arg("--uppercase")
    .arg("--non-hyphenated")
    .assert()
    .success()
    .stdout(predicate::str::is_match(r"^[0-9A-F]{32}\n$").unwrap());
```

**Regex Output Validation:**
- UUID V4: `[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}`
- Non-hyphenated uppercase: `[0-9A-F]{32}`
- NanoID: `[A-Za-z0-9_-]{21}`

**Dev Dependencies Added:**
```toml
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
```

#### Rationale

1. **Completeness:** Unit tests verify internal logic; integration tests verify user-facing behavior
2. **Regression Prevention:** CLI changes are caught by integration tests before release
3. **Documentation:** Integration tests serve as usage examples for end users
4. **Confidence:** 84 total tests provide high confidence in correctness

#### Verification

✅ All 54 unit tests pass  
✅ All 30 integration tests pass  
✅ Clean build with no warnings  
✅ Edge cases covered (overflow, negative values, malformed input)  
✅ Exit codes validated (success=0, error=failure)  

#### Edge Cases Discovered

- **V6 Seed Lenient Parsing:** Accepts seeds with invalid values if exactly 6 valid u8 values found
- **Count=0 Behavior:** Produces no output but exits successfully (loop `1..=0`)
- **NanoID Length=0:** Library hangs (external behavior, test removed)
- **Template Errors:** Gracefully fall back to UUID with stderr warning

#### Files Modified

- `uuidgen/src/main.rs` - Added 8 edge case unit tests
- `uuidgen/tests/integration_tests.rs` - Created with 30 CLI black-box tests
- `uuidgen/Cargo.toml` - Added assert_cmd and predicates dev-dependencies

#### Recommendation for Future CLI Tools

Adopt this dual test strategy as standard pattern:
1. Inline unit tests for internal functions
2. Separate `tests/integration_tests.rs` for CLI black-box testing
3. Use assert_cmd and predicates crates for CLI validation
4. Test both success and failure scenarios
5. Validate exit codes, stdout, stderr independently

---

### Decision: Text Parameter Converted to Option Flag in hashcalc

**Date:** Current session  
**Author:** Marcus (Backend Dev)  
**Status:** Implemented ✓  
**Scope:** CLI refactor

#### Summary

Refactored the hashcalc CLI to accept text input via a `-t` / `--text` option flag instead of as a positional argument.

#### Before (Positional Argument)
```bash
hashcalc "hello world"              # text as positional arg
hashcalc "hello" -a sha1            # text positional, algo option
```

#### After (Option Flag)
```bash
hashcalc -t "hello world"           # text via -t flag
hashcalc -t "hello" -a sha1         # text as option, algo option
```

#### Changes

1. **CLI Definition** - Modified `Args` struct in `src/bin/hashcalc/main.rs`:
   - Added `#[arg(short, long)]` attribute to `text: Option<String>` field
   - Converted from positional parameter to optional flag-based parameter

2. **Test Suite** - Updated all 67 integration tests:
   - Replaced positional text arguments with `-t TEXT` syntax
   - No changes to test logic, only command invocation syntax
   - All tests maintain same coverage and validation

3. **Behavior Preserved**:
   - Mutual exclusivity: `-t` and `-f` still cannot both be provided
   - Both required: error if neither `-t` nor `-f` provided
   - Algorithm selection: `-a / --algorithm` option unchanged
   - Default algorithm: SHA256

#### Rationale

- **Consistency**: All inputs now specified via flags (no positional args)
- **Clarity**: `hashcalc -t "text"` vs `hashcalc -f file` makes both modes explicit
- **Scalability**: If additional positional arguments are added in future, no conflicts
- **Help text**: Auto-generated help is clearer with named options

#### Impact

- **Breaking Change**: Scripts using `hashcalc "text"` must update to `hashcalc -t "text"`
- **All 67 tests passing**: Verified in debug and release modes
- **Zero compiler warnings**: Clean build output

#### Verification

✅ All 67 integration tests pass  
✅ Build: Clean, zero warnings  
✅ Manual spot checks: SHA1, SHA256, algorithm combinations verified  
✅ Help text: Correctly displays `-t, --text <TEXT>`  
✅ Error handling: Mutual exclusivity and missing args still enforced  

---

### Decision: Standardize hashcalc CLI Output Format

**Date:** Current session  
**Author:** Marcus (Backend Dev)  
**Status:** Implemented ✓

Update hashcalc CLI output to a single-line human-friendly format:
```
{input_identifier} [{algorithm}] : {hash_output}
```

#### Details

- For text input (-t), the identifier is the raw text provided.
- For file input (-f), the identifier is the filename only (not full path).
- Algorithm is printed in lowercase.
- No behavioral change to hashing logic or algorithms.

#### Rationale

- Improves readability for users and scripts
- Distinguishes input identifier and algorithm
- Keeps hashes easy to parse (hash is last token)

#### Impact

- CLI output format is backward-incompatible for callers that expected raw hash only. Tests were updated accordingly.

---

### Decision: Add --write Output Sidecar Feature to hashcalc

**Date:** Current session  
**Author:** Marcus (Backend Dev)  
**Status:** Implemented ✓

Add `-w` / `--write` option to hashcalc that writes the computed hash to a sidecar file named `{input_filename}.{algorithm}` in the same directory as the input file.

#### Rationale

- Provides a convenient way to persist hashes next to source files for later verification.
- Keeps output format identical to stdout for consistency.

#### Implementation Details

- CLI flag: `-w` / `--write` boolean added to Args in `src/bin/hashcalc/main.rs`
- Validation: `--write` requires `--file` and cannot be used with `--text`
- Output filename pattern: `{input_filename}.{algorithm_lowercase}` (uses only the basename)
- Error handling: prints descriptive error and exits with code 1 on misuse or write failures

#### Verification

✅ Implemented and covered by integration tests

---

### Decision: Comprehensive hashcalc Test Suite Expansion

**Date:** 2026-04-06  
**Decided by:** Blake (Tester)  
**Status:** ✅ Complete

#### Context

The `hashcalc` workspace had 70 existing unit tests in `main.rs` covering basic CLI functionality and the `hash_content` dispatcher. However, there were no:
- Black-box CLI integration tests
- Per-hasher unit tests with known cryptographic test vectors
- Tests for the individual hasher implementations (sha1.rs, sha256.rs, etc.)

Following the pattern established with `uuidgen`, comprehensive test coverage was needed.

#### Decision

Expanded the test suite from 70 to 147 total tests by adding:

1. **39 Integration Tests** in new file `hashcalc/tests/integration_tests.rs`
   - Black-box CLI testing using `assert_cmd` and `predicates` crates
   - Tests all CLI flags, algorithms, error cases, exit codes
   - Validates file I/O, write flag, output format

2. **38 Per-Hasher Unit Tests** across 5 hasher files
   - Added test modules to: sha1.rs (6), sha256.rs (7), sha512.rs (6), md5.rs (7), base64.rs (12)
   - Known cryptographic test vectors validate correctness
   - Edge cases: empty input, binary data, large data (1MB), unicode

3. **Added Dev Dependencies** to `Cargo.toml`
   - `assert_cmd = "2.0"` for spawning CLI binary in tests
   - `predicates = "3.0"` for flexible output assertions

#### Rationale

- **Industry Best Practice:** Known test vectors (NIST, RFC standards) ensure hash implementations are correct
- **Black-Box Testing:** Integration tests validate the complete CLI user experience
- **Consistency with uuidgen:** Same testing pattern (unit + integration) for maintainability
- **Regression Prevention:** Comprehensive coverage prevents future breakage
- **Quality Assurance:** Every hasher algorithm validated independently

#### Verification

✅ 147 tests passing (108 unit + 39 integration)  
✅ All hashers validated with known cryptographic vectors  
✅ Complete CLI black-box coverage  
✅ Edge cases covered (unicode, binary, large files, error paths)  
✅ Clean build with no warnings

#### Files Modified

- `hashcalc/Cargo.toml` - Added dev-dependencies
- `hashcalc/src/hashers/sha1.rs` - Added 6 tests
- `hashcalc/src/hashers/sha256.rs` - Added 7 tests
- `hashcalc/src/hashers/sha512.rs` - Added 6 tests
- `hashcalc/src/hashers/md5.rs` - Added 7 tests
- `hashcalc/src/hashers/base64.rs` - Added 12 tests
- `hashcalc/tests/integration_tests.rs` - NEW FILE with 39 tests

---

### Directive: Tests in Separate Files (2026-04-21)

**By:** Martin Smith (via Copilot)

Tests must be placed in separate files named `{source_file}_tests.rs`, not inline in source files.

For source files at root of `src/` (e.g., `main.rs`):
```rust
#[cfg(test)] mod main_tests;  // resolves to src/main_tests.rs
```

For non-root source files (e.g., `hashers/base64.rs`):
```rust
#[cfg(test)] #[path = "base64_tests.rs"] mod tests;  // sibling in same dir
```

**Rationale:** Maintains clean separation between implementation and tests across all source structures.

---

### Decision: Standard about/long_about Format for CLI Binaries

**Date:** Session 6  
**Author:** Marcus (Backend Dev)  
**Status:** Implemented ✓  
**Affected Components:** `hashcalc/src/main.rs`, `uuidgen/src/main.rs`

Standardised the `#[command(...)]` `about` and `long_about` fields across all CLI binaries.

**Standard Format:**
```rust
about = concat!("{app_name} v", env!("CARGO_PKG_VERSION"), " - {description}"),
long_about = concat!("{app_name} v", env!("CARGO_PKG_VERSION"), " - {description}\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
```

- **`about`** — shown in brief usage / error output: `{app_name} v{version} - {description}`
- **`long_about`** — shown with `--help`: same as `about` plus the copyright line

**Rationale:**
- Users see which tool and version they're using immediately
- Copyright belongs in `--help` output, not error messages
- `env!("CARGO_PKG_VERSION")` is zero-cost compile-time constant

**Verification:**
✅ Clean build, zero warnings  
✅ 231 tests pass across both binaries

---

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction


---

# Decision: bannertext Test Strategy

**Date:** Current session  
**Author:** Blake (Tester)  
**Status:** Implemented ✓

## Context

The `bannertext` app was already implemented with 23 unit tests in `src/main_tests.rs`. The task was to add integration (black-box CLI) and output (exact string) tests using the same patterns as uuidgen and hashcalc.

## Decision: Two-File Test Split

Split tests into `integration_tests.rs` (behavioural) and `output_tests.rs` (exact output) matching the repo convention.

### integration_tests.rs — 35 tests
Tests observable behaviour without hardcoding exact strings where possible:
- Uses `predicate::str::contains` for content checks
- Uses `lines().collect()` + `assert_eq!(lines.len(), N)` for line count checks
- Uses per-line char checks (`all(|c| c == '*')`) for header/footer line validation

### output_tests.rs — 10 tests
Tests exact byte-for-byte output using Rust's format! to compute expected strings dynamically:
- `format!("**  {:^72}  **", "Hello World")` for center-alignment (captures Rust's right-bias on odd remainder)
- `format!("**  {:>12}  **", "Hi")` for right-alignment
- Hardcoded strings only where length is trivially small ("*******************", "--  X  --", etc.)

## Key Verification Step

All expected outputs were verified by running the binary before writing tests. This eliminated ambiguity around:
1. Whether version string uses "v" prefix in `--version` output (it does NOT: "bannertext 0.1.0-dev"), but help uses "bannertext v0.1.0-dev"
2. Rust center-format left-vs-right bias for odd remainder (extra space goes RIGHT)
3. Trailing spaces when suffix_count=0 (line ends with spaces, not stars)

## Result

68 total tests (23 unit + 35 integration + 10 output) — all green on first run.


---

# Decision: CI Coverage Job Strategy

**Author:** Blake (Tester/QA)  
**Date:** Current session  
**Status:** Proposed

## Decision

Added a `coverage` job to `.github/workflows/ci-build.yml` using `cargo-llvm-cov`.

## Key Choices Made

### 1. Runs on `ubuntu-latest`, not `windows-latest`
Coverage tooling (LLVM instrumentation) is faster and simpler on Linux. The build job owns Windows validation; the coverage job is a QA concern and Linux is the right platform for it.

### 2. Runs in parallel with `build` (both depend on `setup` only)
Coverage is independent of artifact production. Making it depend on `build` would delay feedback. Parallel execution keeps CI fast.

### 3. `tag` and `release` do NOT depend on `coverage`
Release gating is `build`-only. Coverage is informational — a low % should not block a release at this stage. This can be tightened later.

### 4. No minimum coverage threshold enforced
The project has no existing coverage baseline. Enforcing a threshold before the team knows what coverage looks like would be premature. Thresholds should be added once a baseline is established.

### 5. Used `taiki-e/install-action@cargo-llvm-cov` (official action)
This is the canonical installation path for `cargo-llvm-cov`, maintained by the tool author. Avoids `cargo install` compile time in CI.

### 6. LCOV output + step summary
- `lcov.info` uploaded as artifact for future Codecov/Coveralls integration
- `--summary-only` text written to `$GITHUB_STEP_SUMMARY` for inline visibility in GitHub UI

## Future Considerations
- Add Codecov or Coveralls integration consuming `lcov.info`
- Add a minimum coverage threshold once baseline is known
- Consider combining with PR comments showing coverage diff


---

### 2026-05-13T09:33:34Z: User directive
**By:** Martin Smith (via Copilot)
**What:** Do NOT auto-commit after agent work. Scribe should skip the git add .squad/ && git commit step. Ask Martin before committing, or omit commits entirely unless explicitly requested.
**Why:** User request — captured for team memory


---

# Decision: bannertext Implementation Patterns

**Author:** Marcus  
**Date:** 2025  
**Status:** Accepted  

## Context

Implementing the `bannertext` CLI app from a C++ reference spec. Several design decisions were made to produce idiomatic Rust code.

## Decisions

### 1. `parse_single_char` custom value_parser for `char` args

**Decision:** Use a custom `value_parser = parse_single_char` function on all char-typed CLI arguments. Specify defaults as `default_value = "*"` (string literal) rather than `default_value_t`.

**Rationale:** clap 4.x has no built-in `char` parser. The custom function provides a clear error message when the user provides a multi-character or empty string.

### 2. `generate_banner` as a `pub` function

**Decision:** Make `generate_banner` and `TextAlignment` public, with `main_tests.rs` as a module included via `#[cfg(test)] mod main_tests;`.

**Rationale:** Consistent with the uuidgen/hashcalc pattern. Keeps tests colocated without requiring a separate crate. The `pub` keyword enables unit tests to call the function directly.

### 3. Text truncation uses `chars().take(n)`

**Decision:** When `max_total_length` forces `text_area_width` below the text's character count, truncate using `text.chars().take(text_area_width).collect::<String>()`.

**Rationale:** Byte-slicing (`&text[..n]`) can panic on multi-byte UTF-8 boundaries. Char iteration is safe and handles all Unicode text correctly.

### 4. Repeated-char string building

**Decision:** Use `std::iter::repeat(char).take(n).collect::<String>()` everywhere a repeated-character string is needed.

**Rationale:** Idiomatic Rust; more explicit and composable than `String::from_utf8(vec![c as u8; n])` or format tricks.

### 5. Workspace member ordering

**Decision:** Added `bannertext` after `hashcalc` in the workspace `members` array.

**Rationale:** Alphabetical ordering within purpose groups (existing apps first, new app appended). No functional impact.

---

# Decision: printformat Implementation

**Author:** Marcus (Backend Dev)  
**Date:** Current session  
**Status:** Implemented ✓

## Summary

Implemented a new workspace member named `printformat` as a Rust CLI utility that formats text by replacing `{}` placeholders with positional arguments and prints the final string with `println!`.

## Decisions

- Followed the existing app conventions used by `uuidgen`, `hashcalc`, and `bannertext` for `Cargo.toml`, `build.rs`, clap metadata, help/version flags, and test layout.
- Used a dedicated `apply_format(&str, &[String]) -> Result<String, String>` function for placeholder counting and ordered `replacen("{}", arg, 1)` substitution.
- Enforced exact placeholder/argument count matching; mismatches write a clear `Error:` message to stderr and exit with code `1`.
- Added unit tests for `apply_format`, CLI integration tests, and exact-output fixture tests for help and representative formatting cases.
- Added `printformat` to the workspace members list and copied `uuidgen\app.ico` for Windows resource embedding consistency.

## Verification

✓ `cargo build -p printformat`  
✓ `cargo test -p printformat --quiet`  
✓ `cargo test --workspace --quiet`

---

# Decision: printformat Alignment Implementation

**Author:** Marcus (Backend Dev)  
**Date:** Current session  
**Status:** Implemented ✓  
**Impact:** Medium — Extends printformat feature set with formatting support

## Summary

Extended `printformat` to support Rust `std::fmt`-style string alignment, fill, and width handling by preprocessing unnamed placeholders into numbered fields and rendering with `strfmt`.

## Decisions

- Added `strfmt = "0.2.5"` to `printformat\Cargo.toml` so formatting is delegated to a library that already understands Rust-like string alignment syntax.
- Replaced the `replacen("{}", ...)` formatter in `printformat\src\main.rs` with a two-step approach: preprocess the format string, then call `strfmt` with a numbered variable map.
- Auto-number only unnamed placeholders (`{}` and `{:spec}`) from left to right while leaving explicit placeholders like `{0}` and `{1:>5}` unchanged.
- Preserve escaped braces as `{{` and `}}`, and surface malformed brace syntax early with clear errors for unclosed `{` and bare `}`.
- Keep argument validation tied to the count of auto-numbered placeholders, matching the current CLI contract where each CLI argument supplies one unnamed slot.
- Treat supported formatting as string formatting only; numeric-specific formatting remains out of scope because CLI arguments are passed through as strings.

## Verification

✓ `cargo build -p printformat`  
✓ `cargo test -p printformat`  
✓ `cargo test --workspace --quiet`

---

# Decision: printformat Alignment Tests

**Author:** Blake (Tester)  
**Date:** Current session  
**Status:** Implemented ✓

## Summary

Added comprehensive alignment test suite for `printformat` covering Rust-style alignment, fill characters, and width specifications.

## Implementation

- Added an integration-test section for alignment and padding behavior in `printformat/tests/printformat/integration_tests.rs`.
- Covered right, left, center, fill-char, width-only, mixed-format, multi-argument, and escaped-brace CLI scenarios.
- Added two exact-output snapshot tests in `printformat/tests/printformat/output_tests.rs` backed by new `.example` files for right-align and fill-char center output.
- Updated the existing help-output snapshot to include the new alignment examples shown by clap help text.

## Validation

✓ Verified Marcus's implementation readiness by checking `strfmt` in `Cargo.toml` and `preprocess_format` in `src/main.rs` before testing.  
✓ Ran `cargo test -p printformat --quiet` successfully.

---

# Decision: printformat Test Coverage

**Author:** Blake (Tester)  
**Date:** Current session  
**Status:** Implemented ✓

## Summary

Expanded test coverage for `printformat` with focused unit tests and comprehensive CLI integration tests.

## Implementation

- Added focused `apply_format` unit tests for placeholder substitution, empty input, adjacent placeholders, and mismatch errors.
- Expanded CLI integration coverage to include help/version aliases, happy paths, edge cases, clap failure on missing required args, and explicit exit-code checks.
- Kept exact-output snapshot coverage in `.example` files for help text plus representative zero-, one-, and multi-argument formatting scenarios.
- Used `--help` in exact-output tests for portability; `-?` remains covered in integration tests.

## Validation

✓ Validation was run with `cargo test` from `printformat\` once the implementation was present.

---

# Decision: Dependency Version Upgrades

**Author:** Marcus (Backend Dev)  
**Date:** Current session  
**Status:** Implemented ✓

## Summary

Updated workspace manifest minimum versions for several crates and regenerated `Cargo.lock`.

## Changes

- Updated workspace manifest minimum versions only where a newer published crate version was confirmed.
- Left `sha1`, `sha2`, `tempfile`, `winresource`, `build-print`, `hex`, `md5`, and `strfmt` unchanged because the currently declared versions are already the latest published releases.
- Bumped manifest minimums for `clap`, `assert_cmd`, `predicates`, `regex`, `uuid`, and `nanoid`, then regenerated `Cargo.lock` with `cargo update`.

## Verification

✓ Verified the dependency refresh with `cargo build --workspace` and `cargo test --workspace`; all crates still compile and all 345 tests pass.

---

# Decision: Research – C# Format String Support

**Author:** Marcus (Backend Dev)  
**Date:** Current session  
**Status:** Investigation Complete  
**Impact:** Medium — Foundation for C# format translation feature

## Problem Statement

Martin Smith requested investigation into whether `printformat` can support C# style format strings like `{0:D3}`, `{0:F2}`, `{0,-10}`, etc.

## Research Findings

**No C# format-specific Rust crates exist** on crates.io. Searched alternatives: `dyn-fmt`, `formatx`, `dyf` — all Rust-style only.

### Current Support

**printformat already handles:**
- `{0}` `{1}` numeric positional placeholders ✅
- `{}` auto-indexed placeholders ✅
- Rust-style alignment: `{:>10}`, `{:<10}`, `{:^11}` ✅
- Fill chars: `{:*^11}` ✅
- Zero-padding: `{:0>5}` ✅

### Translatable vs. Non-Translatable C# Specs

| C# Format | Translatable | Rust Equivalent | Status |
|-----------|-------------|-----------------|--------|
| `{0,-10}` | ✅ Yes | `{0:<10}` | Syntax differs, semantics map |
| `{0,10}` | ✅ Yes | `{0:>10}` | Right-align, width 10 |
| `{0:D3}` | ✅ Yes | `{0:0>3}` | Integer zero-pad |
| `{0:F2}` | ✅ Yes | `{0:.2}` | Float precision |
| `{0:X}` | ❌ No | N/A | Hexadecimal not in strfmt |
| `{0:N}` | ❌ No | N/A | Thousands separators |
| `{0:C}` | ❌ No | N/A | Currency formatting |
| `{0:G}` | ❌ No | N/A | General numeric format |

**Coverage:** ~50% of common C# specs are translatable.

## Recommended Approach

✅ **Implement a C# → Rust Format Translator** as a lightweight preprocessing layer (~100 lines) fitting cleanly into existing `preprocess_format()` function.

---

# Decision: printformat C# Translation Layer

**Author:** Marcus (Backend Dev)  
**Date:** Current session  
**Status:** Implemented ✓  
**Impact:** High — Enables C# format string compatibility in printformat

## Summary

Implemented opt-in C# format translation layer with `--csharp` / `-c` flag for preprocessing C# format strings before Rust rendering.

## Implementation Details

### CLI Flag
- Added opt-in `--csharp` / `-c` boolean flag to prevent breaking changes to default formatting behavior.
- Flag triggers `translate_csharp_format()` preprocessing before standard formatting.

### Translation Function
- Implemented `translate_csharp_format(format_str: &str) -> Result<String, String>` as pure preprocessing step in `printformat\src\main.rs`.
- Lazy regex compilation for efficiency in repeated calls.
- Kept existing `apply_format(&str, &[String]) -> Result<String, String>` signature intact.

### Supported Translations

- `{0}` / `{1}` → passthrough
- `{0,-N}` → `{0:<N}` (left alignment)
- `{0,N}` → `{0:>N}` (right alignment)
- `{0:DN}` / `{0:dN}` → `{0:0>N}` (decimal zero-padded)
- `{0:FN}` / `{0:fN}` → `{0:.N}` (floating-point precision)
- `{0:EN}` / `{0:eN}` → `{0:.NE}` / `{0:.Ne}` (scientific notation, best-effort)
- `{0:G}` / `{0:g}` → passthrough (general format)

### Unsupported Specifiers (Explicit Error)

- `X` / `x` — hexadecimal
- `N` / `n` — thousands separator
- `C` / `c` — currency

## Verification

✓ `cargo build` — Clean build, zero warnings  
✓ `cargo test` — 38 existing tests + integration passing  
✓ Updated CLI help examples and expected help-output fixtures

---

# Decision: printformat C# Format Tests

**Author:** Blake (Tester)  
**Date:** Current session  
**Status:** Implemented ✓

## Summary

Added 17 comprehensive tests for C# format translation covering translation function, CLI flag integration, and end-to-end format application.

## Test Suite

### Translation Function Tests (12 tests)
- Passthrough: index-only and general format specifiers
- Alignment: left/right alignment with width
- Numeric: zero-padded format
- Floating: decimal place precision
- Scientific: exponential notation
- Error cases: unsupported specifiers

### CLI Integration Tests (3 tests)
- `--csharp` flag parsing and boolean activation
- Format application with flag enabled
- Multi-argument format with C# translation

### Apply Format Integration (2 tests)
- End-to-end C# format translation and rendering
- Edge cases: empty args, missing placeholders

## Verification

✓ `cargo test` — All 17 new tests passing  
✓ `cargo test` — All 38 existing tests still passing  
✓ Total: 55+ tests passing  
✓ Zero warnings, clean build

## Test Design Principles

1. Isolation: Each test focuses on single translation or integration concern
2. Error handling: Invalid C# specifiers caught early with descriptive messages
3. End-to-end: CLI flag, translation, and format application tested together
4. Regression: Existing tests remain passing, no side effects

---

### 2026-05-21T10:13:52Z: User Directive

**By:** Martin Smith (via Copilot)  
**What:** After every code change, run `cargo fmt` to ensure the code is standards compliant before considering the work done.  
**Why:** User request — captured for team memory

---

### 2026-05-23T11:18:55Z: User Directive

**By:** Martin Smith (via Copilot)  
**What:** Do not edit .example files — they are generated via a script  
**Why:** User request — captured for team memory

