use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// ===== CLI Help and Version Tests =====

#[test]
fn test_cli_help_flag() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Generate a hash of text or file contents",
        ))
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
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ));
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
        .stdout(predicate::str::contains(
            "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d",
        ));
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
        .stdout(predicate::str::contains(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ));
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
        .stdout(predicate::str::contains(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
        ));
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
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ));
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
        .stdout(predicate::str::contains(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ));
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
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ));
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
        .code(2)
        .stderr(predicate::str::contains("--file"));
}

#[test]
fn test_cli_write_with_text_fails() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--write")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("--file"));
}

// ===== Error Handling Tests =====

#[test]
fn test_cli_no_args_error() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("--text"));
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
        .code(2)
        .stderr(predicate::str::contains("cannot be used with"));
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
        .code(2)
        .stderr(predicate::str::contains("possible values"));
}

#[test]
fn test_cli_algorithm_case_insensitive() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("SHA256")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ))
        .stdout(predicate::str::contains("[sha256]"));
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
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ));
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
    cmd.arg("--text").arg("hello").assert().code(0);
}

#[test]
fn test_cli_help_exit_code() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--help").assert().code(0);
}

#[test]
fn test_cli_version_exit_code() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--version").assert().code(0);
}

#[test]
fn test_cli_writes_to_closed_pipe_without_panicking() {
    use std::process::{Command as StdCommand, Stdio};

    let mut child = StdCommand::new(env!("CARGO_BIN_EXE_hashcalc"))
        .args(["--text", "hello"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start hashcalc");

    // Close the read end of the pipe before the child can write.
    drop(child.stdout.take());

    let output = child
        .wait_with_output()
        .expect("failed to wait for hashcalc");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "expected success on a closed pipe, got {:?}: {}",
        output.status,
        stderr
    );
    assert!(!stderr.contains("panicked"), "stderr was: {}", stderr);
}

#[test]
fn test_cli_error_exit_code() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    cmd.arg("--text")
        .arg("hello")
        .arg("--algorithm")
        .arg("invalid")
        .assert()
        .code(2);
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

// ===== CLI Tests (moved from the unit tests) =====

fn extract_hash(output: &str) -> String {
    output.trim().rsplit(" : ").next().unwrap().to_string()
}

fn create_temp_file(filename: &str, content: &[u8]) -> PathBuf {
    let path = std::env::current_dir()
        .unwrap()
        .join(format!("test_cli_{}", filename));
    fs::write(&path, content).unwrap();
    path
}

fn cleanup_temp_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_cli_text_mode_hello() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

#[test]
fn test_cli_text_mode_empty_string() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", ""])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
}

#[test]
fn test_cli_text_mode_hello_world() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello world"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));
}

#[test]
fn test_cli_text_mode_special_chars() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "!@#$%^&*()"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 64);
}

#[test]
fn test_cli_algorithm_sha1() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello", "-a", "sha1"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"));
}

#[test]
fn test_cli_algorithm_md5() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello", "-a", "md5"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_cli_algorithm_sha512() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello", "-a", "sha512"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 128);
}

#[test]
fn test_cli_algorithm_base64() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello", "-a", "base64"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aGVsbG8="));
}

#[test]
fn test_cli_algorithm_invalid() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello", "-a", "invalid"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("possible values"), "stderr was: {}", stderr);
}

#[test]
fn test_cli_write_flag_writes_file() {
    let path = create_temp_file("write_test.txt", b"hello");
    let abs_path = std::fs::canonicalize(&path).unwrap();
    let dir = std::fs::canonicalize(".").unwrap();

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", abs_path.to_str().unwrap(), "-a", "sha1", "-w"])
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
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-w", "-t", "hello"])
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

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "hello", "-w"])
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

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-f", path.to_str().unwrap()])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_file_and_text_mutual_exclusivity_error() {
    let path = create_temp_file("conflict.txt", b"test");

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "text", "--file", path.to_str().unwrap()])
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
fn test_cli_file_not_found_error() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", "/nonexistent/path/to/file.txt"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:") || stderr.contains("File not found"));
}

#[test]
fn test_cli_exit_code_success() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "test"])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_cli_exit_code_file_error() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", "/nonexistent/file.txt"])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_cli_exit_code_mutual_exclusivity_error() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "text", "--file", "/tmp/file.txt"])
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_exit_code_no_args_error() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .output()
        .expect("Failed to run hashcalc");

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_consistency_same_input() {
    let output1 = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "consistency"])
        .output()
        .expect("Failed to run hashcalc");

    let output2 = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "consistency"])
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

    let text_output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "equivalence"])
        .output()
        .expect("Failed to run hashcalc");

    let file_output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", abs_path.to_str().unwrap()])
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
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "format_test"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);

    assert_eq!(hex.len(), 64);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_cli_text_different_inputs_different_hashes() {
    let output1 = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "input1"])
        .output()
        .expect("Failed to run hashcalc");

    let output2 = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "input2"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output1.status.success());
    assert!(output2.status.success());
    assert_ne!(
        String::from_utf8_lossy(&output1.stdout),
        String::from_utf8_lossy(&output2.stdout)
    );
}

#[test]
fn test_cli_file_mode_with_sha1() {
    let path = create_temp_file("sha1_file.txt", b"test");

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", path.to_str().unwrap(), "-a", "sha1"])
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

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", path.to_str().unwrap(), "-a", "md5"])
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

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", path.to_str().unwrap(), "-a", "sha512"])
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

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", path.to_str().unwrap(), "-a", "base64"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aGVsbG8="));
    cleanup_temp_file(&path);
}

#[test]
fn test_cli_long_option_algorithm() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "test", "--algorithm", "sha1"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 40);
}

#[test]
fn test_cli_text_with_multiple_spaces() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "   multiple   spaces   "])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hash = extract_hash(&stdout);
    assert_eq!(hash.trim().len(), 64);
}

#[test]
fn test_cli_text_numeric_string() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "1234567890"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hex = extract_hash(&stdout);
    assert_eq!(hex.len(), 64);
}

#[test]
fn test_cli_text_single_character() {
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "x"])
        .output()
        .expect("Failed to run hashcalc");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let hash = extract_hash(&stdout);
    assert_eq!(hash.trim().len(), 64);
}

#[test]
fn test_cli_invalid_algorithm_with_file() {
    let path = create_temp_file("invalid_algo.txt", b"test");

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", path.to_str().unwrap(), "-a", "xyz"])
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
        let output = Command::cargo_bin("hashcalc")
            .unwrap()
            .args(["-t", text, "-a", algo])
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
    let output_default = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "test"])
        .output()
        .expect("Failed to run hashcalc");

    let output_explicit = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "test", "-a", "sha256"])
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
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", "/nonexistent/path/file.txt"])
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

    let output1 = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", abs_path.to_str().unwrap(), "-a", "sha256"])
        .output()
        .expect("Failed to run hashcalc");

    let output2 = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["--file", abs_path.to_str().unwrap(), "-a", "sha1"])
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
    let output = Command::cargo_bin("hashcalc")
        .unwrap()
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

    let output = Command::cargo_bin("hashcalc")
        .unwrap()
        .args(["-t", "text_content", "--file", path.to_str().unwrap()])
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
