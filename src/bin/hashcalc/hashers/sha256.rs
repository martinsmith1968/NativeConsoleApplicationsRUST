use sha2::{Sha256, Digest};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
