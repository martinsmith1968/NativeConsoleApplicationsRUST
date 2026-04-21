use sha1::{Digest, Sha1};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[cfg(test)]
#[path = "sha1_tests.rs"]
mod tests;
