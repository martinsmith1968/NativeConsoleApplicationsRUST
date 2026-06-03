# Marcus Backend Dev - History & Learnings

## Session 5: hashcalc CLI Refactor - Text Parameter to Option Flag

### Refactoring Complete

Converted the hashcalc CLI from accepting text as a positional argument to accepting it via a `-t` / `--text` option flag.

#### Changes Made

1. **Args Struct Modification** - Added `#[arg(short, long)]` attribute to the `text` field in the Args struct, changing it from a positional argument to an optional flag parameter.

2. **Test Suite Update** - Updated all 67 tests to use the new `-t TEXT` syntax instead of positional text arguments:
   - Text mode tests: `cargo run -- "text"` → `cargo run -- -t "text"`
   - Algorithm combination tests: `cargo run -- "text" -a algo` → `cargo run -- -t "text" -a algo`
   - Mutual exclusivity tests: adjusted to use `-t` flag

3. **CLI Behavior Preserved**:
   - `-t "hello world"` hashes the text (default SHA256)
   - `-f file.txt` reads and hashes the file
   - `-a sha1` / `--algorithm sha256` specify hash algorithm
   - Mutual exclusivity enforced: error if both `-t` and `-f` provided
   - Error if neither `-t` nor `-f` provided
   - Help text automatically updated by clap: shows `-t, --text <TEXT>`

#### Testing & Verification

✅ All 67 integration tests pass without modification (after updating command syntax)  
✅ Build: Clean compilation, zero warnings  
✅ Manual verification:
  - `hashcalc -t "hello world"` → Correct SHA256 hash
  - `hashcalc -t hello -a sha1` → Correct SHA1 hash
  - `hashcalc` (no args) → Error with clear message
  - `hashcalc -t text --file file.txt` → Error for mutual exclusivity

#### Key Learnings

- **clap flag syntax**: `#[arg(short, long)]` on Option field converts positional to flag-based option
- **Test maintenance**: When CLI arguments change, all test command invocations must be updated systematically
- **Mutual exclusivity**: The logic for handling `-t` and `-f` remains the same; clap handles argument parsing details
- **Backward incompatibility**: This is a breaking change to the CLI interface — users must update scripts to use `-t` flag

### Key Files Modified
- `src/bin/hashcalc/main.rs` - Args struct, all 67 test commands

### Build Status
✅ Compiles without warnings or errors  
✅ All 67 tests pass  
✅ Manual spot checks verified  
✅ Help text correctly shows `-t, --text <TEXT>`

### Session Outcome
**Refactoring:** COMPLETED ✅  
**Testing:** All 67 tests passing ✅  
**Ready for commit:** YES ✅

---

## Session 4: hashcalc Module Refactoring

### Refactoring Complete

Extracted all five hash algorithms from the monolithic `main.rs` into a clean module structure under `src/bin/hashcalc/hashers/`:

#### New Module Structure
- `hashers/mod.rs` - Module index and `hash_content()` dispatcher
- `hashers/sha1.rs` - SHA1 implementation (40-char hex output)
- `hashers/md5.rs` - MD5 implementation (32-char hex output)
- `hashers/sha256.rs` - SHA256 implementation (64-char hex output)
- `hashers/sha512.rs` - SHA512 implementation (128-char hex output)
- `hashers/base64.rs` - Base64 encoding (custom impl)

#### Key Design Decisions

1. **Consistent public API** - Each hasher module exports `pub fn hash(data: &[u8]) -> Result<String, String>` with uniform error handling.

2. **Centralized dispatch** - `hashers::hash_content()` routes algorithm selection to the correct module, preserving original API compatibility.

3. **Trait imports** - Imported `Digest` trait in sha1.rs, sha256.rs, and sha512.rs to access `.new()`, `.update()`, and `.finalize()` methods. No conflicts despite multi-version digest dependency.

4. **Minimal main.rs** - Reduced main.rs to ~70 lines handling only CLI parsing and I/O orchestration. All algorithm logic is now in dedicated modules.

#### Code Organization Benefits

- **Modularity**: Each algorithm is independently testable and maintainable
- **Clarity**: Readers can understand one algorithm at a time
- **Extensibility**: Adding a new algorithm is now a single new file, not inline code
- **Separation of concerns**: CLI logic separated from hashing logic

#### Testing

✅ All 67 integration tests pass without modification (backward compatibility preserved)
✅ Build: Clean compilation, zero warnings
✅ Manual verification: SHA256, SHA1, Base64 tested and working
✅ File and text modes work correctly with all algorithms

#### Implementation Notes

- Used re-export pattern: `pub use self::sha1::hash as sha1;` to provide convenient namespace
- Match expression in dispatcher is exhaustive, compiler ensures all algorithms handled
- Each hasher independently handles its own dependencies (sha1 crate, sha2 crate, custom encoding)
- Error handling consistent: `Result<String, String>` throughout

### Key Files Modified
- `src/bin/hashcalc/main.rs` - Simplified, imports from hashers module
- Created: `src/bin/hashcalc/hashers/mod.rs` (dispatcher)
- Created: `src/bin/hashcalc/hashers/sha1.rs`
- Created: `src/bin/hashcalc/hashers/md5.rs`
- Created: `src/bin/hashcalc/hashers/sha256.rs`
- Created: `src/bin/hashcalc/hashers/sha512.rs`
- Created: `src/bin/hashcalc/hashers/base64.rs`

### Build Status
✅ Compiles without warnings or errors
✅ All 67 tests pass
✅ Zero breaking changes to CLI interface
✅ No regressions detected

### Session Outcome
**Refactoring:** COMPLETED ✅  
**Code Review:** APPROVED by Kiefer ✅  
**Decisions:** Documented and merged to decisions.md ✅  
**Ready for commit:** YES ✅

---

## Session 1: uuidgen Code Quality Fixes

### Issues Fixed

1. **V6 seed parsing bug (line 132)** - Parsed seed values were assigned to a shadowed `_node` variable that was never used. Fixed by mutating the `node` array with the parsed seed values, so V6 UUID generation now respects the user-provided seed.

2. **Template error crash (line 184)** - `.unwrap()` on `strfmt()` call would panic on malformed templates. Replaced with `match` to gracefully handle errors, logging to stderr and falling back to unformatted UUID.

3. **Cloning overhead (lines 104, 110, 157)** - Removed unnecessary `.clone()` calls on owned values being returned. Functions now directly return owned values from match expressions or macro invocations.

4. **Type limitation (line 37)** - Changed `count: u8` to `count: u32` to support generation of more than 255 UUIDs.

5. **Control flow clarity (lines 90-114)** - Replaced if-return chains with idiomatic `match` expression on `args.uuid_type`, eliminating early returns and improving readability.

6. **Function signatures (line 175, 184)** - Changed `format_output()` parameters from `&String` to `&str`, more flexible and idiomatic. Also changed `sequence: u8` to `u32` to match the new count type.

### Learnings & Patterns

- **Seed Application**: When parsing external input into a fixed-size array, directly mutate the target array rather than creating intermediate references.
- **Error Handling**: Prefer `match` on `Result` types over `.unwrap()` to provide graceful degradation and user feedback.
- **Return Value Optimization**: Owned values should be returned directly from match arms, not cloned after extraction.
- **Parameter Types**: Use `&str` instead of `&String` in function signatures for better ergonomics and zero-cost abstractions.
- **Match over If-Else**: Rust enums are best handled with exhaustive match expressions for type safety and clarity.

### Key Files
- `src/bin/uuidgen/main.rs` - All changes applied here

### Build Status
✅ Compiles without warnings or errors
✅ V6 seed now correctly applies parsed node values
✅ Template errors handled gracefully
✅ Generated 300 UUIDs successfully (u32 count works)

## Session 2: hashcalc File Input Feature

### Implementation

Modified `src/bin/hashcalc/main.rs` to add optional file hashing capability:

1. **Optional text parameter** - Changed `text: String` to `text: Option<String>`, making text input optional.

2. **File input option** - Added `file: Option<String>` with `#[arg(short, long)]` to support `-f` / `--file` flag.

3. **Idiomatic error handling** - Implemented `read_file_contents()` function using `fs::read()` with context-aware error mapping:
   - `NotFound`: "File not found: {path}"
   - `PermissionDenied`: "Permission denied: {path}"
   - `InvalidData`: "Invalid file data: {path}"
   - Default: Generic error with message

4. **Input validation** - Match expression on `(&args.text, &args.file)` tuple:
   - Text only: hash text bytes
   - File only: read and hash file bytes
   - Both: error with exit code 1
   - Neither: error with exit code 1

5. **Help text** - Updated doc comment and clap directives for clarity.

### Testing

✅ Text input: `hashcalc "hello world"` produces correct SHA256
✅ File input: `hashcalc --file testfile.txt` reads and hashes file contents
✅ File not found: Graceful error message, exit code 1
✅ Both arguments: Clear error message preventing ambiguity
✅ No arguments: Prompts user to provide either option
✅ Build: Zero warnings/errors

### Learnings & Patterns

- **Optional inputs with validation**: Use tuple matching on multiple `Option` types for exhaustive input validation.
- **Idiomatic error mapping**: `fs::read().map_err()` with `io::ErrorKind` match for user-friendly messages.
- **Content normalization**: Work with `Vec<u8>` for both text and file content to maintain uniform hashing logic.
- **Exit codes**: Use `std::process::exit(1)` for user input errors (distinct from panics).

## Session 3: hashcalc Multi-Algorithm Support

### Feature Implemented

Extended hashcalc CLI to support multiple hash algorithms via `-a/--algorithm` option. Feature was already partially implemented but had a critical trait import conflict that prevented compilation.

### Fix Applied

**Import Conflict Resolution**: SHA1 crate (0.10.6) and SHA2 crate (0.11.0-rc.5) depend on different versions of the `digest` crate, causing trait resolution failures when importing `Digest` from both.

**Solution**: Import each crate's `Digest` trait with different aliases:
```rust
use sha2::{Sha256, Sha512};
use sha2::Digest as Sha2Digest;
use sha1::Sha1;
```

This allows both trait methods to be available through explicit qualification when needed (though pattern matching on hashers doesn't require explicit trait use).

### Algorithms Supported

- `sha1` - 40-char hex output
- `md5` - 32-char hex output  
- `sha256` - 64-char hex output (default)
- `sha512` - 128-char hex output
- `base64` - base64 encoding of raw bytes

### CLI Features

- Short flag: `-a`, long flag: `--algorithm`
- Default value: `sha256` (maintains backward compatibility)
- Works with both text mode and file mode (`-f`)
- Clear error messages for unsupported algorithms
- All existing functionality preserved

### Testing

✅ Build: Clean compilation without warnings  
✅ Spot checks:
- `cargo run --bin hashcalc -- "hello" -a sha256` → Correct SHA256
- `cargo run --bin hashcalc -- "hello" -a sha1` → Correct SHA1  
- `cargo run --bin hashcalc -- "hello" -a md5` → Correct MD5
- `cargo run --bin hashcalc -- "hello" -a sha512` → 128-char output
- `cargo run --bin hashcalc -- "hello" -a base64` → aGVsbG8=
- `cargo run --bin hashcalc -- "hello"` → Default SHA256 matches explicit
- `cargo run --bin hashcalc -- "hello" -a invalid` → Helpful error message
- File mode: `cargo run --bin hashcalc -- -f file.txt -a sha1` → Works correctly

✅ Comprehensive test suite included (40+ tests covering all algorithms, file/text modes, edge cases, error handling)

### Key Files Modified

- `src/bin/hashcalc/main.rs` - Import conflict resolution only (feature already implemented)

## Session 9: bannertext Multi-Text Support

### Implementation Complete

Changed `bannertext` to accept multiple positional text arguments displayed as separate lines inside one header/footer wrapper.

#### Changes Made

1. `Args.message_text: String` → `Vec<String>` with `#[arg(num_args(1..))]` and `required = true`
2. `generate_banner(text: &str, ...)` → `generate_banner(texts: &[&str], ...)`
3. Width: `texts.iter().map(|t| t.chars().count()).max().unwrap_or(0)` across all texts
4. Text loop: each entry in `texts` gets its own formatted line between the same header/footer
5. Empty slice: `unwrap_or(0)` guards against empty input; no panic
6. `main`: builds `Vec<&str>` from `Vec<String>` via `.iter().map(String::as_str).collect()`

#### Build Status
✅ Clean build, zero warnings  
✅ `bannertext "Hello" "World"` → correct 2-line output with consistent width  

#### Session Outcome
**Implementation:** COMPLETED ✅  
**Integration:** Blake's 86 tests ready to merge (18 new multi-text tests + 68 regression)  
**Documentation:** Decisions merged to .squad/decisions/decisions.md ✅  
**Orchestration Log:** .squad/orchestration-log/2026-05-13T10:44:20Z-marcus.md ✅

---

## Learnings

- Architecture: keep CLI parsing minimal in main.rs and delegate hashing to `src/bin/hashcalc/hashers/*`. This separation made the output-format change low risk.
- Patterns: prefer small focused functions (e.g., read_file_contents) and central dispatcher `hash_content()` for algorithms.
- User preferences: human-friendly single-line output format `{input} [{algorithm}] : {hash}`; algorithm shown lowercase; file outputs use filename only.
- Key paths:
  - src/bin/hashcalc/main.rs (CLI + orchestration)
  - src/bin/hashcalc/hashers/ (algorithm implementations)
  - .squad/agents/marcus/history.md (this file)

- Added --write (-w) option to hashcalc to write output to a sidecar file named `{input_filename}.{algorithm}` in the same directory as the input. Implemented in src\bin\hashcalc\main.rs and covered by integration tests.
- Output format for written files mirrors stdout: `{filename} [{algorithm}] : {hash}`. Errors surface with clear messages and non-zero exit codes on misuse or write failures.
- Standard `about`/`long_about` pattern: `about` = `"{app_name} v{version} - {description}"` for brief usage; `long_about` appends the copyright line so it only shows with `--help`. Use `env!("CARGO_PKG_VERSION")` alongside `env!("BUILD_YEAR")`.
- Dependency upgrade sweep: `cargo search` confirmed `sha1 = 0.11.0`, `sha2 = 0.11.0`, `tempfile = 3.27.0`, `winresource = 0.1.31`, `build-print = 1.0.1`, `hex = 0.4.3`, `md5 = 0.8.0`, and `strfmt = 0.2.5` were already at latest published versions.
- Updated workspace manifest minimums where newer releases existed: `clap 4.6.1`, `assert_cmd 2.2.2`, `predicates 3.1.4`, `regex 1.12.3`, `uuid 1.23.1`, and `nanoid 0.5.0`.
- `cargo update`, `cargo build --workspace`, and `cargo test --workspace` all completed successfully after the upgrade; full workspace test count was 345 passing tests with zero failures.
- New CLI app pattern: mirror the existing workspace app layout (`Cargo.toml`, `build.rs`, `src/main.rs`, `src/main_tests.rs`, `tests/<app>.rs`, `tests/<app>/*`, `tests/ExpectedOutput/*`) when adding a new utility crate.
- `printformat` uses a small `apply_format(&str, &[String]) -> Result<String, String>` helper in `printformat\src\main.rs` so placeholder counting and ordered replacement stay testable and isolated from clap parsing.
- Help/output fixtures for new apps should be validated against real clap output; `printformat` help omits `[OPTIONS]` because only custom help/version actions exist.
- Key paths for this app: `printformat\src\main.rs`, `printformat\src\main_tests.rs`, `printformat\tests\printformat\integration_tests.rs`, and `printformat\tests\ExpectedOutput\*`.

## Session 6: about/long_about Format Standardisation

Updated `#[command(...)]` attributes in both binaries to use the new standard `about`/`long_about` split.

### Files Modified
- `hashcalc/src/main.rs`
- `uuidgen/src/main.rs`

### Build & Test Status
✅ Clean build, zero warnings
✅ 231 tests pass (108 + 39 hashcalc, 54 + 30 uuidgen)

---

## Session 7: bannertext New CLI App

### Implementation Complete

Created the `bannertext` CLI app from scratch following the C++ reference spec.

#### Files Created
- `bannertext/Cargo.toml` — dependencies: clap 4.6.0 only; dev-deps: assert_cmd, predicates, regex; build-deps: winresource, build-print
- `bannertext/build.rs` — identical to uuidgen pattern (graceful missing icon handling)
- `bannertext/src/main.rs` — full implementation
- `bannertext/src/main_tests.rs` — 23 unit tests

#### Key Design Decisions
1. **`parse_single_char` custom value_parser** — clap has no built-in `char` type support; used a small parser function applied via `value_parser = parse_single_char` on each char arg with `default_value = "*"` (not `default_value_t`).
2. **`TextAlignment` as `clap::ValueEnum`** — `#[clap(rename_all = "PascalCase")]` to accept `Left`, `Right`, `Center` on CLI.
3. **`generate_banner` is `pub`** — exposed for unit testing without integration overhead.
4. **Text truncation** — when `max_total_length` shrinks `text_area_width` below text length, text is truncated using `.chars().take(n).collect::<String>()` to handle multi-byte chars safely.
5. **`text_area_width` guard** — if total_length < prefix_total + suffix_total, text_area_width is set to 0, preventing underflow.
6. **Repeated-char strings** — used `std::iter::repeat(char).take(n).collect::<String>()` throughout for idiomatic Rust.

#### Algorithm Verified
"Hello World" default: natural_length = 2+2+11+2+2 = 19. Header = `*******************`. Text = `**  Hello World  **` (19 chars). ✓

#### Build & Test Status
✅ Clean build, zero warnings (expected: missing app.ico warning from build.rs)
✅ 23/23 unit tests pass
✅ Smoke test output matches spec exactly

---

## Session 8: bannertext Complete Implementation

### Project Complete

Created a fully-featured CLI application from scratch that replicates the C++ BannerText specification with 68 total tests passing.

#### Files Created/Modified
- `bannertext/Cargo.toml` — workspace binary with clap 4.6.0, build/dev dependencies
- `bannertext/build.rs` — build script following uuidgen pattern (graceful icon handling)
- `bannertext/src/main.rs` — 200+ line implementation with all features
- `bannertext/src/main_tests.rs` — 23 comprehensive unit tests
- Root `Cargo.toml` — updated workspace members to include bannertext

#### Implementation Highlights

**Custom Parser:**
- `parse_single_char()` value_parser for clap (clap lacks native char type)
- Default value strategy: `default_value = "*"` on each single-char arg (not `default_value_t`)

**TextAlignment Enum:**
- Implements `clap::ValueEnum` with PascalCase names (Left/Right/Center)
- Maps cleanly to CLI: `bannertext -A Center "text"`

**Algorithm Validated:**
- "Hello World" default: 2+2+11+2+2 = 19-char natural width
- Header/footer: `*******************` (19 chars)
- Text: `**  Hello World  **` (19 chars, center-aligned)
- ✓ Exact match to spec output

**Safe String Handling:**
- Multi-byte chars: `.chars().take(n).collect::<String>()` prevents UTF-8 corruption
- Underflow guard: `text_area_width = max(0, ...)` prevents negative widths
- Repeated chars: `std::iter::repeat(char).take(n).collect::<String>()`

**Public API:**
- `generate_banner()` function exposed for unit testing
- Struct fields all public for test assertion clarity
- No need for integration tests (unit tests sufficient)

#### Testing Strategy

23 unit tests cover:
- Default parameters
- Custom header/footer characters and line counts
- All three text alignment modes (Left/Right/Center)
- Min/max length constraints and edge cases
- Prefix/suffix styling combinations
- Unicode and multi-byte character handling

**Integration by Blake:** 35 integration + 10 output tests for CLI validation (separate from this module)

#### Build & Test Status
✅ Clean build, zero warnings
✅ 23/23 unit tests pass
✅ 45 additional integration/output tests passing (Blake's work)
✅ 68 total tests (23 unit + 35 integration + 10 output)
✅ Workspace builds cleanly
✅ Ready for production commit

