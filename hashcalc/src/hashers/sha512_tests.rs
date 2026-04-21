use super::*;

#[test]
fn test_sha512_known_vector_hello() {
    assert_eq!(
        hash(b"hello").unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_sha512_known_vector_empty() {
    assert_eq!(
        hash(b"").unwrap(),
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
    );
}

#[test]
fn test_sha512_known_vector_abc() {
    assert_eq!(
        hash(b"abc").unwrap(),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
    );
}

#[test]
fn test_sha512_binary_data() {
    let data = vec![0x00, 0x01, 0x02, 0xFF];
    let result = hash(&data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 128); // SHA512 produces 128 hex chars
}

#[test]
fn test_sha512_large_data() {
    let large_data = vec![b'a'; 1_000_000];
    let result = hash(&large_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 128);
}

#[test]
fn test_sha512_unicode() {
    let result = hash("hello世界".as_bytes()).unwrap();
    assert_eq!(result.len(), 128);
}
