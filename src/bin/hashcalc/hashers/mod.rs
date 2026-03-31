pub mod sha1;
pub mod md5;
pub mod sha256;
pub mod sha512;
pub mod base64;

pub use self::sha1::hash as sha1;
pub use self::md5::hash as md5;
pub use self::sha256::hash as sha256;
pub use self::sha512::hash as sha512;
pub use self::base64::hash as base64;

pub fn hash_content(content_bytes: &[u8], algorithm: &str) -> Result<String, String> {
    match algorithm {
        "sha1" => self::sha1(content_bytes),
        "md5" => self::md5(content_bytes),
        "sha256" => self::sha256(content_bytes),
        "sha512" => self::sha512(content_bytes),
        "base64" => self::base64(content_bytes),
        _ => Err(format!(
            "Unknown algorithm: '{}'. Supported: sha1, md5, sha256, sha512, base64",
            algorithm
        )),
    }
}
