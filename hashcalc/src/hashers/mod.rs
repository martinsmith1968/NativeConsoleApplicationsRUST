pub mod base64;
pub mod md5;
pub mod sha1;
pub mod sha256;
pub mod sha512;

use std::io::{self, Read};

/// The size of the buffer used when hashing a stream
const BUFFER_SIZE: usize = 64 * 1024;

/// Read `reader` in fixed-size chunks, passing each chunk to `consume`.
fn read_chunks(
    reader: &mut dyn Read,
    mut consume: impl FnMut(&[u8]) -> io::Result<()>,
) -> io::Result<()> {
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }
        consume(&buffer[..bytes_read])?;
    }
}

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

    /// Hash or encode everything read from `reader`, without buffering it all in memory.
    pub fn hash_reader(self, reader: &mut dyn Read) -> io::Result<String> {
        match self {
            Algorithm::Sha1 => self::sha1::hash_reader(reader),
            Algorithm::Md5 => self::md5::hash_reader(reader),
            Algorithm::Sha256 => self::sha256::hash_reader(reader),
            Algorithm::Sha512 => self::sha512::hash_reader(reader),
            Algorithm::Base64 => self::base64::hash_reader(reader),
        }
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Hash or encode an in-memory byte slice.
pub fn hash_content(content_bytes: &[u8], algorithm: Algorithm) -> String {
    algorithm
        .hash_reader(&mut &content_bytes[..])
        .expect("reading from a byte slice cannot fail")
}
