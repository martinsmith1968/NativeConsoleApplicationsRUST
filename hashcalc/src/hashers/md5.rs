pub fn hash(data: &[u8]) -> Result<String, String> {
    let digest = md5::compute(data);
    Ok(format!("{:x}", digest))
}

#[cfg(test)]
#[path = "md5_tests.rs"]
mod tests;
