use sha2::{Digest, Sha512};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[cfg(test)]
#[path = "sha512_tests.rs"]
mod tests;
