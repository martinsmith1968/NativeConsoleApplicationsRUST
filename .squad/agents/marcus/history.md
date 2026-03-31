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

