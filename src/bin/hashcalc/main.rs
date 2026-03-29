use clap::Parser;
use sha2::{Sha256, Sha512};
use sha2::Digest as Sha2Digest;
use sha1::Sha1;
use md5;
use std::fs;
use std::io;

// Notes:
// - https://mojoauth.com/hashing/sha-256-in-rust#validating-and-testing-sha-256-in-rust
// - https://ssojet.com/hashing/sha-256-in-rust

/// Generate a hash of text or file contents
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    author,
    help_expected = true,
)]
struct Args {
    /// The text to generate a hash for (mutually exclusive with --file)
    text: Option<String>,

    /// Path to file to hash
    #[arg(short, long)]
    file: Option<String>,

    /// Hash algorithm to use: sha1, md5, sha256, sha512, base64
    #[arg(short, long, default_value = "sha256")]
    algorithm: String,
}

fn read_file_contents(path: &str) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => format!("File not found: {}", path),
            io::ErrorKind::PermissionDenied => format!("Permission denied: {}", path),
            io::ErrorKind::InvalidData => format!("Invalid file data: {}", path),
            _ => format!("Failed to read file '{}': {}", path, e),
        }
    })
}

fn encode_base64(data: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let mut i = 0;

    while i < data.len() {
        let b1 = data[i];
        let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

        result.push(CHARSET[((n >> 18) & 63) as usize] as char);
        result.push(CHARSET[((n >> 12) & 63) as usize] as char);

        if i + 1 < data.len() {
            result.push(CHARSET[((n >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }

        if i + 2 < data.len() {
            result.push(CHARSET[(n & 63) as usize] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

fn hash_content(content_bytes: &[u8], algorithm: &str) -> Result<String, String> {
    match algorithm {
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(content_bytes);
            let result = hasher.finalize();
            Ok(hex::encode(result))
        }
        "md5" => {
            let digest = md5::compute(content_bytes);
            Ok(format!("{:x}", digest))
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(content_bytes);
            let result = hasher.finalize();
            Ok(hex::encode(result))
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(content_bytes);
            let result = hasher.finalize();
            Ok(hex::encode(result))
        }
        "base64" => {
            let encoded = encode_base64(content_bytes);
            Ok(encoded)
        }
        _ => Err(format!(
            "Unknown algorithm: '{}'. Supported: sha1, md5, sha256, sha512, base64",
            algorithm
        )),
    }
}

fn main() {
    let args = Args::parse();

    let content_bytes = match (&args.text, &args.file) {
        (Some(text), None) => text.as_bytes().to_vec(),
        (None, Some(file_path)) => match read_file_contents(file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        (Some(_), Some(_)) => {
            eprintln!("Error: specify either text or --file, not both");
            std::process::exit(1);
        },
        (None, None) => {
            eprintln!("Error: provide either text or --file option");
            std::process::exit(1);
        },
    };

    match hash_content(&content_bytes, &args.algorithm) {
        Ok(hash_output) => println!("{}", hash_output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::Command;

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
        let hex = hash_content(&bytes, "sha256").unwrap();
        
        // Known SHA256 hash of "hello world"
        assert_eq!(hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn test_hash_content_empty_string() {
        let text = "";
        let bytes = text.as_bytes().to_vec();
        let hex = hash_content(&bytes, "sha256").unwrap();
        
        // Known SHA256 hash of empty string
        assert_eq!(hex, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_hash_content_hello() {
        let text = "hello";
        let bytes = text.as_bytes().to_vec();
        let hex = hash_content(&bytes, "sha256").unwrap();
        
        // SHA256 of "hello"
        assert_eq!(hex, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test_hash_content_consistent() {
        let text = "consistency test";
        let bytes = text.as_bytes().to_vec();
        
        let hex1 = hash_content(&bytes, "sha256").unwrap();
        let hex2 = hash_content(&bytes, "sha256").unwrap();
        
        assert_eq!(hex1, hex2);
    }

    #[test]
    fn test_hash_content_binary_data() {
        let binary_data = vec![0u8, 1, 2, 3, 255, 254, 253, 252];
        let hex = hash_content(&binary_data, "sha256").unwrap();
        
        assert_eq!(hex.len(), 64);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hash_content_large_data() {
        let large_data = vec![42u8; 1024 * 1024];
        let hex = hash_content(&large_data, "sha256").unwrap();
        
        assert_eq!(hex.len(), 64);
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

    #[test]
    fn test_hash_content_md5() {
        let text = "hello";
        let bytes = text.as_bytes().to_vec();
        let hex = hash_content(&bytes, "md5").unwrap();
        
        // Known MD5 hash of "hello"
        assert_eq!(hex, "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(hex.len(), 32);
    }

    #[test]
    fn test_hash_content_sha512() {
        let text = "hello";
        let bytes = text.as_bytes().to_vec();
        let hex = hash_content(&bytes, "sha512").unwrap();
        
        assert_eq!(hex.len(), 128);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hash_content_base64() {
        let text = "hello";
        let bytes = text.as_bytes().to_vec();
        let encoded = hash_content(&bytes, "base64").unwrap();
        
        // Base64 of "hello"
        assert_eq!(encoded, "aGVsbG8=");
    }

    #[test]
    fn test_hash_content_invalid_algorithm() {
        let text = "hello";
        let bytes = text.as_bytes().to_vec();
        let result = hash_content(&bytes, "invalid");
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown algorithm"));
    }

    // ===== Unit Tests for read_file_contents function =====

    #[test]
    fn test_read_file_simple() {
        let path = create_temp_file("test_read.txt", b"hello world");
        let result = read_file_contents(path.to_str().unwrap());
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"hello world".to_vec());
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_read_file_empty() {
        let path = create_temp_file("test_empty.txt", b"");
        let result = read_file_contents(path.to_str().unwrap());
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<u8>::new());
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_read_file_not_found() {
        let result = read_file_contents("nonexistent_file_12345.txt");
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("File not found"));
    }

    #[test]
    fn test_read_file_binary() {
        let binary_data = vec![0u8, 255, 127, 64];
        let path = create_temp_file("test_binary.bin", &binary_data);
        let result = read_file_contents(path.to_str().unwrap());
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), binary_data);
        cleanup_temp_file(&path);
    }

    // ===== CLI Integration Tests =====

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

    #[test]
    fn test_cli_text_mode_empty_string() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", ""])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
    }

    #[test]
    fn test_cli_text_mode_hello_world() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "hello world"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));
    }

    #[test]
    fn test_cli_text_mode_special_chars() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "!@#$%^&*()"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 64);
    }

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

    #[test]
    fn test_cli_algorithm_md5() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "hello", "-a", "md5"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("5d41402abc4b2a76b9719d911017c592"));
    }

    #[test]
    fn test_cli_algorithm_sha512() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "hello", "-a", "sha512"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 128);
    }

    #[test]
    fn test_cli_algorithm_base64() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "hello", "-a", "base64"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("aGVsbG8="));
    }

    #[test]
    fn test_cli_algorithm_invalid() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "hello", "-a", "invalid"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Unknown algorithm"));
    }

    #[test]
    fn test_cli_file_mode_simple() {
        let path = create_temp_file("cli_test.txt", b"hello");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap()])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_file_mode_short_option() {
        let path = create_temp_file("cli_short.txt", b"test");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "-f", path.to_str().unwrap()])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_file_and_text_mutual_exclusivity_error() {
        let path = create_temp_file("conflict.txt", b"test");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "text", "--file", path.to_str().unwrap()])
            .output()
            .expect("Failed to run hashcalc");

        assert!(!output.status.success());
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
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("provide either text or --file option"));
    }

    #[test]
    fn test_cli_file_not_found_error() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", "/nonexistent/path/to/file.txt"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error:") || stderr.contains("File not found"));
    }

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

    #[test]
    fn test_cli_exit_code_mutual_exclusivity_error() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "text", "--file", "/tmp/file.txt"])
            .output()
            .expect("Failed to run hashcalc");

        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn test_cli_exit_code_no_args_error() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--"])
            .output()
            .expect("Failed to run hashcalc");

        assert_eq!(output.status.code(), Some(1));
    }

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
        assert_eq!(String::from_utf8_lossy(&output1.stdout), String::from_utf8_lossy(&output2.stdout));
    }

    #[test]
    fn test_cli_file_and_text_equivalence() {
        let path = create_temp_file("equiv.txt", b"equivalence");
        let abs_path = std::fs::canonicalize(&path).unwrap();
        
        let text_output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "equivalence"])
            .output()
            .expect("Failed to run hashcalc");

        let file_output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", abs_path.to_str().unwrap()])
            .output()
            .expect("Failed to run hashcalc");

        assert!(text_output.status.success());
        assert!(file_output.status.success());
        assert_eq!(String::from_utf8_lossy(&text_output.stdout), String::from_utf8_lossy(&file_output.stdout));
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_output_format_single_line() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "format_test"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        
        assert_eq!(hex_line.len(), 64);
        assert!(hex_line.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_cli_output_hex_lowercase() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "lowercase_test"])
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
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap()])
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
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap()])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 64);
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_text_different_inputs_different_hashes() {
        let output1 = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "input1"])
            .output()
            .expect("Failed to run hashcalc");

        let output2 = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "input2"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output1.status.success());
        assert!(output2.status.success());
        assert_ne!(String::from_utf8_lossy(&output1.stdout), String::from_utf8_lossy(&output2.stdout));
    }

    // ===== Additional Tests for Enhanced Coverage =====

    #[test]
    fn test_hash_content_very_long_string() {
        let long_string = "a".repeat(10000);
        let bytes = long_string.as_bytes().to_vec();
        let hex = hash_content(&bytes, "sha256").unwrap();
        
        assert_eq!(hex.len(), 64);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hash_content_unicode_string() {
        let text = "Hello 世界 🌍";
        let bytes = text.as_bytes().to_vec();
        let hex = hash_content(&bytes, "sha256").unwrap();
        
        assert_eq!(hex.len(), 64);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hash_content_newlines_and_tabs() {
        let text = "line1\nline2\tcolumn2";
        let bytes = text.as_bytes().to_vec();
        let hex = hash_content(&bytes, "sha256").unwrap();
        
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn test_hash_content_base64_padding() {
        let text = "a";  // Results in base64 with padding
        let bytes = text.as_bytes().to_vec();
        let encoded = hash_content(&bytes, "base64").unwrap();
        
        assert_eq!(encoded, "YQ==");
    }

    #[test]
    fn test_hash_content_base64_no_padding() {
        let text = "abc";  // Results in base64 without padding
        let bytes = text.as_bytes().to_vec();
        let encoded = hash_content(&bytes, "base64").unwrap();
        
        assert_eq!(encoded, "YWJj");
    }

    #[test]
    fn test_hash_content_base64_with_special_bytes() {
        let data = vec![0u8, 1, 2, 3, 255, 254];
        let encoded = hash_content(&data, "base64").unwrap();
        
        // Base64 encoding should be valid
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_hash_content_sha1_consistency() {
        let text = "sha1_test";
        let bytes = text.as_bytes().to_vec();
        
        let hex1 = hash_content(&bytes, "sha1").unwrap();
        let hex2 = hash_content(&bytes, "sha1").unwrap();
        
        assert_eq!(hex1, hex2);
        assert_eq!(hex1.len(), 40);
    }

    #[test]
    fn test_hash_content_md5_consistency() {
        let text = "md5_test";
        let bytes = text.as_bytes().to_vec();
        
        let hex1 = hash_content(&bytes, "md5").unwrap();
        let hex2 = hash_content(&bytes, "md5").unwrap();
        
        assert_eq!(hex1, hex2);
        assert_eq!(hex1.len(), 32);
    }

    #[test]
    fn test_hash_content_sha256_different_lengths() {
        let test_cases = vec!["a", "ab", "abc", "abcd"];
        let mut hashes = Vec::new();
        
        for text in test_cases {
            let bytes = text.as_bytes().to_vec();
            let hex = hash_content(&bytes, "sha256").unwrap();
            hashes.push(hex);
        }
        
        // All should be different
        for i in 0..hashes.len() {
            for j in (i+1)..hashes.len() {
                assert_ne!(hashes[i], hashes[j]);
            }
        }
    }

    #[test]
    fn test_read_file_with_newlines() {
        let path = create_temp_file("file_newlines.txt", b"line1\nline2\nline3\n");
        let result = read_file_contents(path.to_str().unwrap());
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"line1\nline2\nline3\n".to_vec());
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_read_file_large() {
        let large_content = vec![65u8; 1024 * 100];  // 100KB
        let path = create_temp_file("file_large.bin", &large_content);
        let result = read_file_contents(path.to_str().unwrap());
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1024 * 100);
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_file_mode_with_sha1() {
        let path = create_temp_file("sha1_file.txt", b"test");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap(), "-a", "sha1"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 40);
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_file_mode_with_md5() {
        let path = create_temp_file("md5_file.txt", b"test");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap(), "-a", "md5"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 32);
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_file_mode_with_sha512() {
        let path = create_temp_file("sha512_file.txt", b"test");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap(), "-a", "sha512"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 128);
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_file_mode_with_base64() {
        let path = create_temp_file("base64_file.txt", b"hello");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap(), "-a", "base64"])
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
            .args(&["run", "--bin", "hashcalc", "--", "test", "--algorithm", "sha1"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 40);
    }

    #[test]
    fn test_cli_text_with_spaces() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "   multiple   spaces   "])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout.trim().len(), 64);
    }

    #[test]
    fn test_cli_text_numeric_string() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "1234567890"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let hex_line = stdout.trim();
        assert_eq!(hex_line.len(), 64);
    }

    #[test]
    fn test_cli_text_single_character() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "x"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout.trim().len(), 64);
    }

    #[test]
    fn test_cli_algorithm_case_sensitive() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "test", "-a", "SHA256"])
            .output()
            .expect("Failed to run hashcalc");

        // Should fail because algorithm is case-sensitive
        assert!(!output.status.success());
    }

    #[test]
    fn test_cli_invalid_algorithm_with_file() {
        let path = create_temp_file("invalid_algo.txt", b"test");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", path.to_str().unwrap(), "-a", "xyz"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Unknown algorithm"));
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_multiple_algorithms_on_same_text() {
        let text = "consistency_check";
        let mut hashes = Vec::new();
        
        for algo in &["sha256", "sha1", "md5", "sha512"] {
            let output = Command::new("cargo")
                .args(&["run", "--bin", "hashcalc", "--", text, "-a", algo])
                .output()
                .expect("Failed to run hashcalc");
            
            assert!(output.status.success());
            hashes.push(String::from_utf8_lossy(&output.stdout).to_string());
        }
        
        // All hashes should be different
        for i in 0..hashes.len() {
            for j in (i+1)..hashes.len() {
                assert_ne!(hashes[i].trim(), hashes[j].trim());
            }
        }
    }

    #[test]
    fn test_cli_default_algorithm_is_sha256() {
        let output_default = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "test"])
            .output()
            .expect("Failed to run hashcalc");

        let output_explicit = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "test", "-a", "sha256"])
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
            .args(&["run", "--bin", "hashcalc", "--", "--file", "/nonexistent/path/file.txt"])
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
            .args(&["run", "--bin", "hashcalc", "--", "--file", abs_path.to_str().unwrap(), "-a", "sha256"])
            .output()
            .expect("Failed to run hashcalc");

        let output2 = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "--file", abs_path.to_str().unwrap(), "-a", "sha1"])
            .output()
            .expect("Failed to run hashcalc");

        // Both should succeed
        assert!(output1.status.success());
        assert!(output2.status.success());
        
        // Results should be different because algorithms are different
        assert_ne!(
            String::from_utf8_lossy(&output1.stdout).trim(),
            String::from_utf8_lossy(&output2.stdout).trim()
        );
        cleanup_temp_file(&path);
    }

    #[test]
    fn test_cli_text_and_file_both_missing_error() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--"])
            .output()
            .expect("Failed to run hashcalc");

        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(1));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("provide either text or --file option"));
    }

    #[test]
    fn test_cli_text_priority_over_file() {
        // When both text and file are provided, should give error
        let path = create_temp_file("priority.txt", b"file_content");
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "hashcalc", "--", "text_content", "--file", path.to_str().unwrap()])
            .output()
            .expect("Failed to run hashcalc");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("specify either text or --file, not both"));
        cleanup_temp_file(&path);
    }
}
