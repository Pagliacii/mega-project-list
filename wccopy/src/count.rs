use std::cmp;
use std::default::Default;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Read};

use crate::error::{WCError, WCErrorKind};

#[derive(Debug, Default)]
pub struct Counter {
    bytes: usize,
    chars: usize,
    lines: usize,
    words: usize,
    width: usize,
    from: String,
}

impl Counter {
    pub fn set_from(&mut self, source: &str) {
        self.from = String::from(source);
    }

    pub fn get_bytes(&self) -> usize {
        self.bytes
    }

    pub fn get_chars(&self) -> usize {
        self.chars
    }

    pub fn get_lines(&self) -> usize {
        self.lines
    }

    pub fn get_words(&self) -> usize {
        self.words
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn count(&mut self) -> Result<(), WCError> {
        let reader: BufReader<Box<dyn Read>>;
        if "-" == self.from {
            // A. count by inputs from stdin
            reader = BufReader::new(Box::new(io::stdin()));
        } else {
            // B. count by file contents
            let f = match File::open(&self.from) {
                Ok(f) => f,
                Err(e) => {
                    let mut message = format!("failed to open '{}'", self.from);
                    if e.kind() == io::ErrorKind::NotFound {
                        message = format!("'{}' : No such file or directory", self.from);
                    }
                    return Err(WCError::new(1, WCErrorKind::OpenFailed(e), &message));
                }
            };
            reader = BufReader::new(Box::new(f));
        };

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    return Err(WCError::new(
                        1,
                        WCErrorKind::ReadFailed(e),
                        "invalid UTF-8 bytes",
                    ))
                }
            };
            let bytes = line.len() + 1; // add 1 to include the newline character
            self.bytes += bytes;
            self.chars += bytes;
            self.words += line.split_whitespace().count();
            self.width = cmp::max(self.width, line.len());
            self.lines += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn it_works() {
        let mut dir = env::temp_dir();
        dir.push("test_count_file.txt");
        let path = dir.to_str().expect("Unable to get path string");
        let mut buffer = File::create(path).expect("Unable to create the temporary file");
        buffer
            .write_all(b"a bc def\nghi jklmn\n")
            .expect("Unable to write the temporary file");

        let mut counter: Counter = Default::default();
        counter.set_from(path);
        let result = counter.count();
        fs::remove_file(path).unwrap_or_else(|err| println!("Error: {}", err));
        assert!(result.is_ok());
        assert_eq!(2, counter.get_lines());
        assert_eq!(5, counter.get_words());
        assert_eq!(19, counter.get_chars());
        assert_eq!(19, counter.get_bytes());
        assert_eq!(9, counter.get_width());
    }
}
