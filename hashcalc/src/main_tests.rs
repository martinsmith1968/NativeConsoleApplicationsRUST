use super::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn extract_hash(output: &str) -> String {
    output.trim().rsplitn(2, " : ").next().unwrap().to_string()
}

// Helper function to create temporary test files
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

// ===== Unit Tests for hash_content function =====

#[test]
fn test_hash_content_simple_string() {
    let text = "hello world";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha256);

    // Known SHA256 hash of "hello world"
    assert_eq!(
        hex,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
    assert_eq!(hex.len(), 64);
}

#[test]
fn test_hash_content_empty_string() {
    let text = "";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha256);

    // Known SHA256 hash of empty string
    assert_eq!(
        hex,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_hash_content_hello() {
    let text = "hello";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha256);

    // SHA256 of "hello"
    assert_eq!(
        hex,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_hash_content_consistent() {
    let text = "consistency test";
    let bytes = text.as_bytes().to_vec();

    let hex1 = hash_content(&bytes, Algorithm::Sha256);
    let hex2 = hash_content(&bytes, Algorithm::Sha256);

    assert_eq!(hex1, hex2);
}

#[test]
fn test_hash_content_binary_data() {
    let binary_data = vec![0u8, 1, 2, 3, 255, 254, 253, 252];
    let hex = hash_content(&binary_data, Algorithm::Sha256);

    assert_eq!(hex.len(), 64);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_hash_content_large_data() {
    let large_data = vec![42u8; 1024 * 1024];
    let hex = hash_content(&large_data, Algorithm::Sha256);

    assert_eq!(hex.len(), 64);
}

#[test]
fn test_hash_content_sha1() {
    let text = "hello";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha1);

    // Known SHA1 hash of "hello"
    assert_eq!(hex, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    assert_eq!(hex.len(), 40);
}

#[test]
fn test_hash_content_md5() {
    let text = "hello";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Md5);

    // Known MD5 hash of "hello"
    assert_eq!(hex, "5d41402abc4b2a76b9719d911017c592");
    assert_eq!(hex.len(), 32);
}

#[test]
fn test_hash_content_sha512() {
    let text = "hello";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha512);

    assert_eq!(hex.len(), 128);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_hash_content_base64() {
    let text = "hello";
    let bytes = text.as_bytes().to_vec();
    let encoded = hash_content(&bytes, Algorithm::Base64);

    // Base64 of "hello"
    assert_eq!(encoded, "aGVsbG8=");
}

#[test]
fn test_algorithm_names_are_lowercase() {
    assert_eq!(Algorithm::Sha1.to_string(), "sha1");
    assert_eq!(Algorithm::Md5.to_string(), "md5");
    assert_eq!(Algorithm::Sha256.to_string(), "sha256");
    assert_eq!(Algorithm::Sha512.to_string(), "sha512");
    assert_eq!(Algorithm::Base64.to_string(), "base64");
}

#[test]
fn test_cli_algorithm_accepts_any_case() {
    let args = Args::try_parse_from(["hashcalc", "--text", "hello", "--algorithm", "SHA256"])
        .expect("SHA256 should be accepted");
    assert_eq!(args.algorithm, Algorithm::Sha256);
}

#[test]
fn test_cli_requires_text_or_file() {
    assert!(Args::try_parse_from(["hashcalc"]).is_err());
}

#[test]
fn test_cli_rejects_text_and_file_together() {
    let result = Args::try_parse_from(["hashcalc", "--text", "hello", "--file", "somefile.txt"]);
    assert!(result.is_err());
}

#[test]
fn test_cli_write_conflicts_with_text() {
    let result = Args::try_parse_from(["hashcalc", "--text", "hello", "--write"]);
    assert!(result.is_err());
}

// ===== Unit Tests for hash_file function =====

#[test]
fn test_hash_file_simple() {
    let path = create_temp_file("test_read.txt", b"hello world");
    let result = hash_file(path.to_str().unwrap(), Algorithm::Sha256);

    assert_eq!(
        result.unwrap(),
        hash_content(b"hello world", Algorithm::Sha256)
    );
    cleanup_temp_file(&path);
}

#[test]
fn test_hash_file_empty() {
    let path = create_temp_file("test_empty.txt", b"");
    let result = hash_file(path.to_str().unwrap(), Algorithm::Sha256);

    assert_eq!(result.unwrap(), hash_content(b"", Algorithm::Sha256));
    cleanup_temp_file(&path);
}

#[test]
fn test_hash_file_not_found() {
    let error = hash_file("nonexistent_file_12345.txt", Algorithm::Sha256).unwrap_err();

    assert!(error.to_string().contains("File not found"));
}

#[test]
fn test_hash_file_binary() {
    let binary_data = vec![0u8, 255, 127, 64];
    let path = create_temp_file("test_binary.bin", &binary_data);
    let result = hash_file(path.to_str().unwrap(), Algorithm::Sha256);

    assert_eq!(
        result.unwrap(),
        hash_content(&binary_data, Algorithm::Sha256)
    );
    cleanup_temp_file(&path);
}

// ===== CLI Integration Tests =====

#[test]
fn test_cli_text_mode_hello() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "hello"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

#[test]
fn test_cli_text_mode_empty_string() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", ""])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
}

#[test]
fn test_cli_text_mode_hello_world() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "hello world"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));
}

#[test]
fn test_cli_text_mode_special_chars() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "!@#$%^&*()"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 64);
}

#[test]
fn test_cli_algorithm_sha1() {
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "hashcalc", "--", "-t", "hello", "-a", "sha1",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"));
}

#[test]
fn test_cli_algorithm_md5() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "hello", "-a", "md5"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_cli_algorithm_sha512() {
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "hashcalc", "--", "-t", "hello", "-a", "sha512",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 128);
}

#[test]
fn test_cli_algorithm_base64() {
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "hashcalc", "--", "-t", "hello", "-a", "base64",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aGVsbG8="));
}

#[test]
fn test_cli_algorithm_invalid() {
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "hashcalc", "--", "-t", "hello", "-a", "invalid",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("possible values"), "stderr was: {}", stderr);
}

#[test]
fn test_cli_file_mode_simple() {
    let path = create_temp_file("cli_test.txt", b"hello");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_write_flag_writes_file() {
    let path = create_temp_file("write_test.txt", b"hello");
    let abs_path = std::fs::canonicalize(&path).unwrap();
    let dir = std::fs::canonicalize(".").unwrap();

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            abs_path.to_str().unwrap(),
            "-a",
            "sha1",
            "-w",
        ])
        .output()
        .expect("Failed to run hashcalc");

    // Should exit success and not print to stdout
    assert!(output.status.success());
    assert!(output.stdout.is_empty());

    // Determine expected output file name based on input file_name
    let input_file_name = path.file_name().and_then(|s| s.to_str()).unwrap();
    let expected_out = format!("{}.{}", input_file_name, "sha1");
    let out_path = dir.join(&expected_out);

    assert!(out_path.exists());
    let contents = fs::read_to_string(&out_path).unwrap();
    assert!(contents.contains(&format!("{} [sha1] :", input_file_name)));

    // cleanup
    let _ = fs::remove_file(out_path);
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_write_without_file_errors() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-w", "-t", "hello"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be used with"),
        "stderr was: {}",
        stderr
    );
}

#[test]
fn test_cli_write_with_text_errors() {
    let path = create_temp_file("write_conflict.txt", b"hello");

    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "hello", "-w"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be used with"),
        "stderr was: {}",
        stderr
    );
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_file_mode_short_option() {
    let path = create_temp_file("cli_short.txt", b"test");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "-f",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_file_and_text_mutual_exclusivity_error() {
    let path = create_temp_file("conflict.txt", b"test");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "-t",
            "text",
            "--file",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be used with"),
        "stderr was: {}",
        stderr
    );
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_no_args_error() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--text"), "stderr was: {}", stderr);
}

#[test]
fn test_cli_file_not_found_error() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            "/nonexistent/path/to/file.txt",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:") || stderr.contains("File not found"));
}

#[test]
fn test_cli_exit_code_success() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "test"])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_cli_exit_code_file_error() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            "/nonexistent/file.txt",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_cli_exit_code_mutual_exclusivity_error() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "-t",
            "text",
            "--file",
            "/tmp/file.txt",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_exit_code_no_args_error() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--"])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_consistency_same_input() {
    let output1 = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "consistency"])
        .output()
        .expect("Failed to run hashcalc");

    let output2 = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "consistency"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output1.status.success());
    assert!(output2.status.success());
    assert_eq!(
        extract_hash(&String::from_utf8_lossy(&output1.stdout)),
        extract_hash(&String::from_utf8_lossy(&output2.stdout))
    );
}

#[test]
fn test_cli_file_and_text_equivalence() {
    let path = create_temp_file("equiv.txt", b"equivalence");
    let abs_path = std::fs::canonicalize(&path).unwrap();

    let text_output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "equivalence"])
        .output()
        .expect("Failed to run hashcalc");

    let file_output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            abs_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(text_output.status.success());
    assert!(file_output.status.success());
    let text_stdout = String::from_utf8_lossy(&text_output.stdout);
    let file_stdout = String::from_utf8_lossy(&file_output.stdout);
    assert_eq!(extract_hash(&text_stdout), extract_hash(&file_stdout));
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_output_format_single_line() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "format_test"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);

    assert_eq!(hex.len(), 64);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_cli_output_hex_lowercase() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "lowercase_test"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex_line = stdout.trim();

    // Hex output should be lowercase
    assert_eq!(hex_line, hex_line.to_lowercase());
}

#[test]
fn test_cli_file_mode_empty_file() {
    let path = create_temp_file("empty_cli.txt", b"");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_file_mode_binary_file() {
    let binary_data = vec![0u8, 255, 127, 64];
    let path = create_temp_file("binary_cli.bin", &binary_data);

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 64);
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_text_different_inputs_different_hashes() {
    let output1 = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "input1"])
        .output()
        .expect("Failed to run hashcalc");

    let output2 = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "input2"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output1.status.success());
    assert!(output2.status.success());
    assert_ne!(
        String::from_utf8_lossy(&output1.stdout),
        String::from_utf8_lossy(&output2.stdout)
    );
}

// ===== Additional Tests for Enhanced Coverage =====

#[test]
fn test_hash_content_very_long_string() {
    let long_string = "a".repeat(10000);
    let bytes = long_string.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha256);

    assert_eq!(hex.len(), 64);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_hash_content_unicode_string() {
    let text = "Hello 世界 🌍";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha256);

    assert_eq!(hex.len(), 64);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_hash_content_newlines_and_tabs() {
    let text = "line1\nline2\tcolumn2";
    let bytes = text.as_bytes().to_vec();
    let hex = hash_content(&bytes, Algorithm::Sha256);

    assert_eq!(hex.len(), 64);
}

#[test]
fn test_hash_content_base64_padding() {
    let text = "a"; // Results in base64 with padding
    let bytes = text.as_bytes().to_vec();
    let encoded = hash_content(&bytes, Algorithm::Base64);

    assert_eq!(encoded, "YQ==");
}

#[test]
fn test_hash_content_base64_no_padding() {
    let text = "abc"; // Results in base64 without padding
    let bytes = text.as_bytes().to_vec();
    let encoded = hash_content(&bytes, Algorithm::Base64);

    assert_eq!(encoded, "YWJj");
}

#[test]
fn test_hash_content_base64_with_special_bytes() {
    let data = vec![0u8, 1, 2, 3, 255, 254];
    let encoded = hash_content(&data, Algorithm::Base64);

    // Base64 encoding should be valid
    assert!(!encoded.is_empty());
}

#[test]
fn test_hash_content_sha1_consistency() {
    let text = "sha1_test";
    let bytes = text.as_bytes().to_vec();

    let hex1 = hash_content(&bytes, Algorithm::Sha1);
    let hex2 = hash_content(&bytes, Algorithm::Sha1);

    assert_eq!(hex1, hex2);
    assert_eq!(hex1.len(), 40);
}

#[test]
fn test_hash_content_md5_consistency() {
    let text = "md5_test";
    let bytes = text.as_bytes().to_vec();

    let hex1 = hash_content(&bytes, Algorithm::Md5);
    let hex2 = hash_content(&bytes, Algorithm::Md5);

    assert_eq!(hex1, hex2);
    assert_eq!(hex1.len(), 32);
}

#[test]
fn test_hash_content_sha256_different_lengths() {
    let test_cases = vec!["a", "ab", "abc", "abcd"];
    let mut hashes = Vec::new();

    for text in test_cases {
        let bytes = text.as_bytes().to_vec();
        let hex = hash_content(&bytes, Algorithm::Sha256);
        hashes.push(hex);
    }

    // All should be different
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(hashes[i], hashes[j]);
        }
    }
}

#[test]
fn test_hash_file_with_newlines() {
    let content = b"line1\nline2\nline3\n";
    let path = create_temp_file("file_newlines.txt", content);
    let result = hash_file(path.to_str().unwrap(), Algorithm::Sha256);

    assert_eq!(result.unwrap(), hash_content(content, Algorithm::Sha256));
    cleanup_temp_file(&path);
}

#[test]
fn test_hash_file_large() {
    let large_content = vec![65u8; 1024 * 100]; // 100KB, larger than one read buffer
    let path = create_temp_file("file_large.bin", &large_content);
    let result = hash_file(path.to_str().unwrap(), Algorithm::Sha256);

    assert_eq!(
        result.unwrap(),
        hash_content(&large_content, Algorithm::Sha256)
    );
    cleanup_temp_file(&path);
}

#[test]
fn test_streamed_file_matches_in_memory_for_all_algorithms() {
    let sizes = [0usize, 1, 2, 3, 1023, 65_535, 65_536, 65_537, 100_000];

    for size in sizes {
        let content: Vec<u8> = (0..size).map(|index| (index % 251) as u8).collect();
        let path = create_temp_file(&format!("stream_{}.bin", size), &content);

        for algorithm in [
            Algorithm::Sha1,
            Algorithm::Md5,
            Algorithm::Sha256,
            Algorithm::Sha512,
            Algorithm::Base64,
        ] {
            assert_eq!(
                hash_file(path.to_str().unwrap(), algorithm).unwrap(),
                hash_content(&content, algorithm),
                "size {size}, algorithm {algorithm}"
            );
        }

        cleanup_temp_file(&path);
    }
}

#[test]
fn test_cli_file_mode_with_sha1() {
    let path = create_temp_file("sha1_file.txt", b"test");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
            "-a",
            "sha1",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 40);
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_file_mode_with_md5() {
    let path = create_temp_file("md5_file.txt", b"test");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
            "-a",
            "md5",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 32);
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_file_mode_with_sha512() {
    let path = create_temp_file("sha512_file.txt", b"test");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
            "-a",
            "sha512",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 128);
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_file_mode_with_base64() {
    let path = create_temp_file("base64_file.txt", b"hello");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
            "-a",
            "base64",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aGVsbG8="));
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_long_option_algorithm() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "-t",
            "test",
            "--algorithm",
            "sha1",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 40);
}

#[test]
fn test_cli_text_with_spaces() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "-t",
            "   multiple   spaces   ",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hash = extract_hash(&stdout);
    assert_eq!(hash.trim().len(), 64);
}

#[test]
fn test_cli_text_numeric_string() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "1234567890"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 64);
}

#[test]
fn test_cli_text_single_character() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "x"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hash = extract_hash(&stdout);
    assert_eq!(hash.trim().len(), 64);
}

#[test]
fn test_cli_algorithm_case_insensitive() {
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "hashcalc", "--", "-t", "test", "-a", "SHA256",
        ])
        .output()
        .expect("Failed to run hashcalc");

    // Algorithm names are matched case-insensitively
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[sha256]"), "stdout was: {}", stdout);
}

#[test]
fn test_cli_invalid_algorithm_with_file() {
    let path = create_temp_file("invalid_algo.txt", b"test");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            path.to_str().unwrap(),
            "-a",
            "xyz",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("possible values"), "stderr was: {}", stderr);
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_multiple_algorithms_on_same_text() {
    let text = "consistency_check";
    let mut hashes = Vec::new();

    for algo in &["sha256", "sha1", "md5", "sha512"] {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "-t", text, "-a", algo])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        hashes.push(extract_hash(&stdout));
    }

    // All hashes should be different
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(hashes[i].trim(), hashes[j].trim());
        }
    }
}

#[test]
fn test_cli_default_algorithm_is_sha256() {
    let output_default = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--", "-t", "test"])
        .output()
        .expect("Failed to run hashcalc");

    let output_explicit = Command::new("cargo")
        .args(&[
            "run", "--bin", "hashcalc", "--", "-t", "test", "-a", "sha256",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output_default.status.success());
    assert!(output_explicit.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output_default.stdout),
        String::from_utf8_lossy(&output_explicit.stdout)
    );
}

#[test]
fn test_cli_file_not_found_message() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            "/nonexistent/path/file.txt",
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("Error"));
}

#[test]
fn test_cli_file_preserves_content_for_multiple_algorithms() {
    let path = create_temp_file("multi_algo.txt", b"content");
    let abs_path = std::fs::canonicalize(&path).unwrap();

    let output1 = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            abs_path.to_str().unwrap(),
            "-a",
            "sha256",
        ])
        .output()
        .expect("Failed to run hashcalc");

    let output2 = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "--file",
            abs_path.to_str().unwrap(),
            "-a",
            "sha1",
        ])
        .output()
        .expect("Failed to run hashcalc");

    // Both should succeed
    assert!(output1.status.success());
    assert!(output2.status.success());

    // Results should be different because algorithms are different
    let out1 = String::from_utf8_lossy(&output1.stdout);
    let out2 = String::from_utf8_lossy(&output2.stdout);
    assert_ne!(extract_hash(&out1), extract_hash(&out2));
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_text_and_file_both_missing_error() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hashcalc", "--"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--text"), "stderr was: {}", stderr);
}

#[test]
fn test_cli_text_priority_over_file() {
    // When both text and file are provided, should give error
    let path = create_temp_file("priority.txt", b"file_content");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "hashcalc",
            "--",
            "-t",
            "text_content",
            "--file",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be used with"),
        "stderr was: {}",
        stderr
    );
    cleanup_temp_file(&path);
}
