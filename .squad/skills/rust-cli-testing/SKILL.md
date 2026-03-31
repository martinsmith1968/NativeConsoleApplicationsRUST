---
name: "Rust CLI Testing Patterns"
description: "Comprehensive testing approaches for Rust CLI applications with mixed unit and integration tests"
domain: "testing, cli-validation, error-handling"
confidence: "high"
source: "earned via hashcalc test suite development"
tools:
  - name: "cargo test"
    description: "Run Rust tests with inline #[cfg(test)] modules"
    when: "Validating functionality after code changes"
---

## Context

Testing Rust CLI applications requires a balanced approach:
- **Unit tests** for individual functions (parsing, hashing, I/O)
- **Integration tests** that spawn the actual binary via `Command::new("cargo")`
- **Edge case coverage** for error scenarios, empty inputs, large data
- **CLI arg validation** for mutual exclusivity and required arguments
- **Output verification** via stdout/stderr capture and exit codes

This skill applies when:
- Writing tests for CLI tools with file I/O, algorithms, or transformations
- Validating error handling and exit codes
- Ensuring consistency across multiple runs and algorithms
- Testing edge cases (empty files, special characters, unicode, binary data)

## Patterns

### 1. Mixed Testing Strategy

**Unit tests** for core logic:
```rust
#[test]
fn test_hash_content_simple_string() {
    let text = "hello world";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, "sha256").unwrap();
    assert_eq!(hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    assert_eq!(hex.len(), 64);
}
```

**Integration tests** that spawn the binary:
```rust
#[test]
fn test_cli_text_mode_hello() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "hello"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}
```

### 2. File Handling for Tests

Create temporary test files with cleanup:
```rust
fn create_temp_file(filename: &str, content: &[u8]) -> PathBuf {
    let path = PathBuf::from(format!("test_{}", filename));
    let absolute_path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir().unwrap().join(&path)
    };
    let mut file = fs::File::create(&absolute_path).unwrap();
    file.write_all(content).unwrap();
    absolute_path
}

fn cleanup_temp_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_cli_file_mode_simple() {
    let path = create_temp_file("cli_test.txt", b"hello");
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap()])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    cleanup_temp_file(&path);
}
```

### 3. Known Hash Validation

Store expected hashes for test inputs:
```rust
#[test]
fn test_hash_content_hello() {
    let text = "hello";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, "sha256").unwrap();
    
    // Known SHA256 hash of "hello"
    assert_eq!(hex, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
}

#[test]
fn test_hash_content_sha1() {
    let text = "hello";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, "sha1").unwrap();
    
    // Known SHA1 hash of "hello"
    assert_eq!(hex, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    assert_eq!(hex.len(), 40);
}
```

### 4. Algorithm Coverage

Test each algorithm with known outputs and length validation:
```rust
#[test]
fn test_cli_algorithm_sha1() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "hello", "-a", "sha1"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"));
}

// Test length of different algorithm outputs
#[test]
fn test_output_lengths() {
    // SHA256: 64 chars
    // SHA1: 40 chars
    // MD5: 32 chars
    // SHA512: 128 chars
}
```

### 5. Error Scenario Testing

Test mutual exclusivity and error codes:
```rust
#[test]
fn test_cli_file_and_text_mutual_exclusivity_error() {
    let path = create_temp_file("conflict.txt", b"test");
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "text", "--file", path.to_str().unwrap()])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("specify either text or --file, not both"));
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_no_args_error() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}
```

### 6. Edge Case Coverage

Test empty inputs, large data, special characters:
```rust
// Empty inputs
#[test]
fn test_hash_content_empty_string() {
    let bytes = "".as_bytes().to_vec();
    let hex = hash_content(&bytes, "sha256").unwrap();
    assert_eq!(hex, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

// Very long strings
#[test]
fn test_hash_content_very_long_string() {
    let long_string = "a".repeat(10000);
    let bytes = long_string.as_bytes().to_vec();
    let hex = hash_content(&bytes, "sha256").unwrap();
    assert_eq!(hex.len(), 64);
}

// Unicode content
#[test]
fn test_hash_content_unicode_string() {
    let text = "Hello 世界 🌍";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, "sha256").unwrap();
    assert_eq!(hex.len(), 64);
}

// Binary data
#[test]
fn test_hash_content_binary_data() {
    let binary_data = vec![0u8, 1, 2, 3, 255, 254, 253, 252];
    let hex = hash_content(&binary_data, "sha256").unwrap();
    assert_eq!(hex.len(), 64);
}

// Large files
#[test]
fn test_read_file_large() {
    let large_content = vec![65u8; 1024 * 100];  // 100KB
    let path = create_temp_file("file_large.bin", &large_content);
    let result = read_file_contents(path.to_str().unwrap());
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1024 * 100);
    cleanup_temp_file(&path);
}
```

### 7. Consistency and Equivalence

Verify same input produces same output and text/file modes match:
```rust
#[test]
fn test_cli_consistency_same_input() {
    let output1 = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "consistency"])
        .output()
        .expect("Failed to run hashcalc");

    let output2 = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "consistency"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output1.status.success());
    assert!(output2.status.success());
    assert_eq!(String::from_utf8_lossy(&output1.stdout), 
               String::from_utf8_lossy(&output2.stdout));
}

#[test]
fn test_cli_file_and_text_equivalence() {
    let path = create_temp_file("equiv.txt", b"equivalence");
    
    let text_output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "equivalence"])
        .output()
        .expect("Failed to run hashcalc");

    let file_output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap()])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(String::from_utf8_lossy(&text_output.stdout), 
               String::from_utf8_lossy(&file_output.stdout));
    cleanup_temp_file(&path);
}
```

### 8. Exit Code Validation

Always check exit codes for error scenarios:
```rust
#[test]
fn test_cli_exit_code_success() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "test"])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_cli_exit_code_file_error() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "--file", "/nonexistent/file.txt"])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(1));
}
```

## Examples

**Complete test module organization:**
- Place all tests in `#[cfg(test)] mod tests { ... }` at end of binary
- Use helper functions for common test setup
- Group related tests with comments
- One concern per test (clear assertion)
- Descriptive test names that explain what is being tested

**Test categories for CLI apps:**
1. Unit tests for core functions (parsing, algorithms, I/O)
2. CLI text mode tests
3. CLI file mode tests
4. Algorithm selection and validation
5. Error and mutual exclusivity
6. Exit codes
7. Output format
8. Edge cases (empty, large, special characters, unicode, binary)
9. Consistency and equivalence

## Anti-Patterns

❌ **Don't:** Mock file I/O when testing file mode - use real temporary files
❌ **Don't:** Test only the happy path - always test error scenarios
❌ **Don't:** Forget to validate exit codes - errors must exit with 1
❌ **Don't:** Mix multiple concerns in one test - keep assertions focused
❌ **Don't:** Hardcode paths without cleanup - always remove test files
❌ **Don't:** Assume specific error messages - verify stderr contains key phrases
❌ **Don't:** Skip consistency checks - same input must always produce same output
❌ **Don't:** Forget unicode, binary, and large data tests - edge cases matter
❌ **Don't:** Run tests that modify global state without cleanup

## Comprehensive Test Checklist

- [ ] Unit tests for all public functions
- [ ] Text/argument parsing tests
- [ ] File I/O tests (read, write, error cases)
- [ ] Error handling (invalid args, missing files, permission denied)
- [ ] Mutual exclusivity (when multiple options conflict)
- [ ] Exit codes (0 for success, 1 for errors)
- [ ] Output format validation (structure, format, encoding)
- [ ] Algorithm/mode selection
- [ ] Edge cases (empty, very large, special chars, unicode, binary)
- [ ] Consistency (same input → same output across runs)
- [ ] Equivalence (different modes produce same result for same content)
- [ ] Integration tests spawning actual binary
- [ ] Performance/scalability (large files, large counts)

