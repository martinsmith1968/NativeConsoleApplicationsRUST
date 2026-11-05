use std::io::{self, Read};

pub fn hash_reader(reader: &mut dyn Read) -> io::Result<String> {
    let mut context = md5::Context::new();
    super::read_chunks(reader, |chunk| {
        context.consume(chunk);
        Ok(())
    })?;
    Ok(format!("{:x}", context.finalize()))
}

#[cfg(test)]
fn hash(data: &[u8]) -> String {
    hash_reader(&mut &data[..]).expect("reading from a byte slice cannot fail")
}

#[cfg(test)]
#[path = "md5_tests.rs"]
mod tests;
