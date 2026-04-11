use sha2::{Digest, Sha256};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_vector_hello() {
        assert_eq!(
            hash(b"hello").unwrap(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha256_known_vector_empty() {
        assert_eq!(
            hash(b"").unwrap(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_sha256_known_vector_abc() {
        assert_eq!(
            hash(b"abc").unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_sha256_known_vector_hello_world() {
        assert_eq!(
            hash(b"hello world").unwrap(),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_sha256_binary_data() {
        let data = vec![0x00, 0x01, 0x02, 0xFF];
        let result = hash(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 64); // SHA256 produces 64 hex chars
    }

    #[test]
    fn test_sha256_large_data() {
        let large_data = vec![b'a'; 1_000_000];
        let result = hash(&large_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 64);
    }

    #[test]
    fn test_sha256_unicode() {
        let result = hash("hello世界".as_bytes()).unwrap();
        assert_eq!(result.len(), 64);
    }
}
