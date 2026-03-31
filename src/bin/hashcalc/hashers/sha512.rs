use sha2::{Sha512, Digest};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
