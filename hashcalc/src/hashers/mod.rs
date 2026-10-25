pub mod base64;
pub mod md5;
pub mod sha1;
pub mod sha256;
pub mod sha512;

pub use self::base64::hash as base64;
pub use self::md5::hash as md5;
pub use self::sha1::hash as sha1;
pub use self::sha256::hash as sha256;
pub use self::sha512::hash as sha512;

/// The hash algorithms supported by hashcalc
#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "lowercase")]
pub enum Algorithm {
    Sha1,
    Md5,
    Sha256,
    Sha512,
    Base64,
}

impl Algorithm {
    /// The lowercase name used in the program output
    pub fn name(self) -> &'static str {
        match self {
            Algorithm::Sha1 => "sha1",
            Algorithm::Md5 => "md5",
            Algorithm::Sha256 => "sha256",
            Algorithm::Sha512 => "sha512",
            Algorithm::Base64 => "base64",
        }
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

pub fn hash_content(content_bytes: &[u8], algorithm: Algorithm) -> Result<String, String> {
    match algorithm {
        Algorithm::Sha1 => self::sha1(content_bytes),
        Algorithm::Md5 => self::md5(content_bytes),
        Algorithm::Sha256 => self::sha256(content_bytes),
        Algorithm::Sha512 => self::sha512(content_bytes),
        Algorithm::Base64 => self::base64(content_bytes),
    }
}
