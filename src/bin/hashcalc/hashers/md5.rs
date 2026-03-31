pub fn hash(data: &[u8]) -> Result<String, String> {
    let digest = md5::compute(data);
    Ok(format!("{:x}", digest))
}
