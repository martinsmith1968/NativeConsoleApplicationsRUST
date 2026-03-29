# Marcus Backend Dev - History & Learnings

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
