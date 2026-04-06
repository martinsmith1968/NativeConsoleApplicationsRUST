use sha1::{Digest, Sha1};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_known_vector_hello() {
        assert_eq!(
            hash(b"hello").unwrap(),
            "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
        );
    }

    #[test]
    fn test_sha1_known_vector_empty() {
        assert_eq!(
            hash(b"").unwrap(),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        );
    }

    #[test]
    fn test_sha1_known_vector_abc() {
        assert_eq!(
            hash(b"abc").unwrap(),
            "a9993e364706816aba3e25717850c26c9cd0d89d"
        );
    }

    #[test]
    fn test_sha1_binary_data() {
        let data = vec![0x00, 0x01, 0x02, 0xFF];
        let result = hash(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 40); // SHA1 produces 40 hex chars
    }

    #[test]
    fn test_sha1_large_data() {
        let large_data = vec![b'a'; 1_000_000];
        let result = hash(&large_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 40);
    }

    #[test]
    fn test_sha1_unicode() {
        let result = hash("hello世界".as_bytes()).unwrap();
        assert_eq!(result.len(), 40);
    }
}
