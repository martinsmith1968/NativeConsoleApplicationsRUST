use sha2::{Digest, Sha256};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
