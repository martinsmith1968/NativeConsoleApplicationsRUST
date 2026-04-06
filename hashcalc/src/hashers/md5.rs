pub fn hash(data: &[u8]) -> Result<String, String> {
    let digest = md5::compute(data);
    Ok(format!("{:x}", digest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_known_vector_hello() {
        assert_eq!(
            hash(b"hello").unwrap(),
            "5d41402abc4b2a76b9719d911017c592"
        );
    }

    #[test]
    fn test_md5_known_vector_empty() {
        assert_eq!(
            hash(b"").unwrap(),
            "d41d8cd98f00b204e9800998ecf8427e"
        );
    }

    #[test]
    fn test_md5_known_vector_abc() {
        assert_eq!(
            hash(b"abc").unwrap(),
            "900150983cd24fb0d6963f7d28e17f72"
        );
    }

    #[test]
    fn test_md5_known_vector_hello_world() {
        assert_eq!(
            hash(b"hello world").unwrap(),
            "5eb63bbbe01eeed093cb22bb8f5acdc3"
        );
    }

    #[test]
    fn test_md5_binary_data() {
        let data = vec![0x00, 0x01, 0x02, 0xFF];
        let result = hash(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32); // MD5 produces 32 hex chars
    }

    #[test]
    fn test_md5_large_data() {
        let large_data = vec![b'a'; 1_000_000];
        let result = hash(&large_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_md5_unicode() {
        let result = hash("hello世界".as_bytes()).unwrap();
        assert_eq!(result.len(), 32);
    }
}
