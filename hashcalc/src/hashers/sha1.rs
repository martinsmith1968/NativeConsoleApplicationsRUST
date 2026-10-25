use sha1::{Digest, Sha1};
use std::io::{self, Read};

pub fn hash_reader(reader: &mut dyn Read) -> io::Result<String> {
    let mut hasher = Sha1::new();
    super::read_chunks(reader, |chunk| {
        hasher.update(chunk);
        Ok(())
    })?;
    Ok(hex::encode(hasher.finalize()))
}

#[cfg(test)]
fn hash(data: &[u8]) -> String {
    hash_reader(&mut &data[..]).expect("reading from a byte slice cannot fail")
}

#[cfg(test)]
#[path = "sha1_tests.rs"]
mod tests;
