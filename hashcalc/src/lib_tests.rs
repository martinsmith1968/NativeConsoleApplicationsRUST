use super::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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
