pub fn hash(data: &[u8]) -> Result<String, String> {
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

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_known_vector_hello() {
        assert_eq!(hash(b"hello").unwrap(), "aGVsbG8=");
    }

    #[test]
    fn test_base64_known_vector_empty() {
        assert_eq!(hash(b"").unwrap(), "");
    }

    #[test]
    fn test_base64_known_vector_f() {
        assert_eq!(hash(b"f").unwrap(), "Zg==");
    }

    #[test]
    fn test_base64_known_vector_fo() {
        assert_eq!(hash(b"fo").unwrap(), "Zm8=");
    }

    #[test]
    fn test_base64_known_vector_foo() {
        assert_eq!(hash(b"foo").unwrap(), "Zm9v");
    }

    #[test]
    fn test_base64_known_vector_foob() {
        assert_eq!(hash(b"foob").unwrap(), "Zm9vYg==");
    }

    #[test]
    fn test_base64_known_vector_fooba() {
        assert_eq!(hash(b"fooba").unwrap(), "Zm9vYmE=");
    }

    #[test]
    fn test_base64_known_vector_foobar() {
        assert_eq!(hash(b"foobar").unwrap(), "Zm9vYmFy");
    }

    #[test]
    fn test_base64_binary_data() {
        let data = vec![0x00, 0x01, 0x02];
        assert_eq!(hash(&data).unwrap(), "AAEC");
    }

    #[test]
    fn test_base64_all_zeros() {
        let data = vec![0x00, 0x00, 0x00];
        assert_eq!(hash(&data).unwrap(), "AAAA");
    }

    #[test]
    fn test_base64_all_ones() {
        let data = vec![0xFF, 0xFF, 0xFF];
        assert_eq!(hash(&data).unwrap(), "////");
    }

    #[test]
    fn test_base64_unicode() {
        let result = hash("hello世界".as_bytes()).unwrap();
        assert!(result.len() > 0);
        assert!(
            result
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
        );
    }
}
