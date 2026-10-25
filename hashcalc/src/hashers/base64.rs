use base64::engine::general_purpose::STANDARD;
use base64::write::EncoderWriter;
use std::io::{self, Read, Write};

pub fn hash_reader(reader: &mut dyn Read) -> io::Result<String> {
    let mut encoder = EncoderWriter::new(Vec::new(), &STANDARD);
    super::read_chunks(reader, |chunk| encoder.write_all(chunk))?;

    let encoded = encoder.finish()?;
    String::from_utf8(encoded).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

#[cfg(test)]
fn hash(data: &[u8]) -> String {
    hash_reader(&mut &data[..]).expect("reading from a byte slice cannot fail")
}

#[cfg(test)]
#[path = "base64_tests.rs"]
mod tests;
