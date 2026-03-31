use sha1::{Sha1, Digest};

pub fn hash(data: &[u8]) -> Result<String, String> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
