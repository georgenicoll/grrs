use anyhow::Result;
use std::io::{BufRead, BufReader, Read, Write};

pub fn find_matches<T>(reader: BufReader<T>, pattern: &str, mut writer: impl Write) -> Result<()>
where
    T: Read
{
    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_matches(BufReader::new("lorem ipsum\ndolor sit amet".as_bytes()), "lorem", &mut result).unwrap();
        assert_eq!(result, b"lorem ipsum\n");
    }

}