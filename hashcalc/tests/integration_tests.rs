use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

// ===== CLI Help and Version Tests =====

#[test]
fn test_cli_help_flag() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate a hash of text or file contents"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version_flag() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("hashcalc"));
}

// ===== Text Mode Tests =====

#[test]
fn test_cli_text_mode_sha256_default() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

#[test]
fn test_cli_text_mode_sha1() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("sha1")
        .assert()
        .success()
        .stdout(predicate::str::contains("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"));
}

#[test]
fn test_cli_text_mode_md5() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("md5")
        .assert()
        .success()
        .stdout(predicate::str::contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_cli_text_mode_sha512() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("sha512")
        .assert()
        .success()
        .stdout(predicate::str::contains("9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"));
}

#[test]
fn test_cli_text_mode_base64() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("base64")
        .assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8="));
}

#[test]
fn test_cli_text_empty_string() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success()
        .stdout(predicate::str::contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
}

#[test]
fn test_cli_text_unicode() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello世界")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success();
}

#[test]
fn test_cli_text_with_spaces() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello world")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success()
        .stdout(predicate::str::contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));
}

#[test]
fn test_cli_text_special_characters() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("!@#$%^&*()")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success();
}

// ===== File Mode Tests =====

#[test]
fn test_cli_file_mode_simple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "hello").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

#[test]
fn test_cli_file_mode_with_algorithm() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "hello").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .arg("--algorithm")
        .arg("md5")
        .assert()
        .success()
        .stdout(predicate::str::contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_cli_file_mode_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.txt");
    fs::write(&file_path, "").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success()
        .stdout(predicate::str::contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
}

#[test]
fn test_cli_file_mode_binary_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("binary.bin");
    fs::write(&file_path, vec![0x00, 0x01, 0x02, 0xFF]).unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success();
}

#[test]
fn test_cli_file_not_found() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg("nonexistent_file.txt")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_cli_file_short_option() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "hello").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

// ===== Write Mode Tests =====

#[test]
fn test_cli_write_flag_creates_hash_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "hello").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .arg("--algorithm")
        .arg("sha256")
        .arg("--write")
        .assert()
        .success();

    let hash_file_path = temp_dir.path().join("test.txt.sha256");
    assert!(hash_file_path.exists());
    let content = fs::read_to_string(&hash_file_path).unwrap();
    assert!(content.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

#[test]
fn test_cli_write_with_md5() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("data.txt");
    fs::write(&file_path, "test data").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .arg("--algorithm")
        .arg("md5")
        .arg("--write")
        .assert()
        .success();

    let hash_file_path = temp_dir.path().join("data.txt.md5");
    assert!(hash_file_path.exists());
}

#[test]
fn test_cli_write_without_file_fails() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--write")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("--write requires --file"));
}

#[test]
fn test_cli_write_with_text_fails() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--write")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("--write requires --file"));
}

// ===== Error Handling Tests =====

#[test]
fn test_cli_no_args_error() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("provide either text or --file"));
}

#[test]
fn test_cli_both_text_and_file_error() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "hello").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--file")
        .arg(&file_path)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("specify either text or --file, not both"));
}

#[test]
fn test_cli_invalid_algorithm() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("invalid_algo")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Unknown algorithm"));
}

#[test]
fn test_cli_algorithm_case_sensitive() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("SHA256")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Unknown algorithm"));
}

// ===== Algorithm Short Option Tests =====

#[test]
fn test_cli_algorithm_short_option() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("-t")
        .arg("hello")
        .arg("-a")
        .arg("md5")
        .assert()
        .success()
        .stdout(predicate::str::contains("5d41402abc4b2a76b9719d911017c592"));
}

// ===== Output Format Tests =====

#[test]
fn test_cli_output_format_contains_text() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("[sha256]"))
        .stdout(predicate::str::contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

#[test]
fn test_cli_output_format_contains_filename() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("myfile.txt");
    fs::write(&file_path, "hello").unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success()
        .stdout(predicate::str::contains("myfile.txt"))
        .stdout(predicate::str::contains("[sha256]"));
}

#[test]
fn test_cli_output_hex_lowercase() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"[0-9a-f]+").unwrap())
        .stdout(predicate::str::is_match(r"[A-F]+").unwrap().not());
}

// ===== Exit Code Tests =====

#[test]
fn test_cli_success_exit_code() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .assert()
        .code(0);
}

#[test]
fn test_cli_help_exit_code() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--help")
        .assert()
        .code(0);
}

#[test]
fn test_cli_version_exit_code() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--version")
        .assert()
        .code(0);
}

#[test]
fn test_cli_error_exit_code() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("invalid")
        .assert()
        .code(1);
}

// ===== Large File Tests =====

#[test]
fn test_cli_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large.txt");
    let large_data = "x".repeat(1_000_000);
    fs::write(&file_path, &large_data).unwrap();

    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--file")
        .arg(&file_path)
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success();
}

// ===== Edge Case Tests =====

#[test]
fn test_cli_newline_in_text() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello\nworld")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success();
}

#[test]
fn test_cli_tab_in_text() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello\tworld")
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success();
}

#[test]
fn test_cli_quotes_in_text() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg(r#""quoted""#)
        .arg("--algorithm")
        .arg("sha256")
        .assert()
        .success();
}

// ===== Consistency Tests =====

#[test]
fn test_cli_text_and_file_same_content_same_hash() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "consistent").unwrap();

    let mut cmd1 = Command::cargo_bin("hashcalc").unwrap();
    let output1 = cmd1.arg("--text").arg("consistent").output().unwrap();
    
    let mut cmd2 = Command::cargo_bin("hashcalc").unwrap();
    let output2 = cmd2.arg("--file").arg(&file_path).output().unwrap();
    
    let hash1 = String::from_utf8(output1.stdout).unwrap();
    let hash2 = String::from_utf8(output2.stdout).unwrap();
    
    // Extract hash values (last part after " : ")
    let hash1_val = hash1.split(" : ").last().unwrap().trim();
    let hash2_val = hash2.split(" : ").last().unwrap().trim();
    
    assert_eq!(hash1_val, hash2_val);
}

#[test]
fn test_cli_multiple_runs_same_hash() {
    let mut cmd1 = Command::cargo_bin("hashcalc").unwrap();
    let output1 = cmd1.arg("--text").arg("stable").output().unwrap();
    
    let mut cmd2 = Command::cargo_bin("hashcalc").unwrap();
    let output2 = cmd2.arg("--text").arg("stable").output().unwrap();
    
    assert_eq!(output1.stdout, output2.stdout);
}
