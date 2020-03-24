use std::default::Default;

/// Controls the behaviors of this program.
///
/// The default behavior is that prints the newline counts, word counts,
/// and byte counts to screen. Just like using the `--lines`,
/// `--words`, and `--bytes` flags.
///
/// Printing order: newline, word, character, byte, maximum line length
#[derive(Debug, Clone)]
pub struct Config {
    // print the byte counts
    byte_counts: bool,
    // print the character counts
    char_counts: bool,
    // print the newline counts
    line_counts: bool,
    // print the word counts
    word_counts: bool,
    // print the maximum display width
    max_width: bool,
    // files to be count
    files: Vec<String>,
}

impl Config {
    /// byte_counts setter
    pub fn byte_toggle(&mut self, enabled: bool) {
        self.byte_counts = enabled;
    }

    /// byte_counts getter
    pub fn is_byte_enable(&self) -> bool {
        self.byte_counts
    }

    /// char_counts setter
    pub fn char_toggle(&mut self, enabled: bool) {
        self.char_counts = enabled;
    }

    /// char_counts getter
    pub fn is_char_enable(&self) -> bool {
        self.char_counts
    }

    /// line_counts setter
    pub fn line_toggle(&mut self, enabled: bool) {
        self.line_counts = enabled;
    }

    /// line_counts getter
    pub fn is_line_enable(&self) -> bool {
        self.line_counts
    }

    /// word_counts setter
    pub fn word_toggle(&mut self, enabled: bool) {
        self.word_counts = enabled;
    }

    /// word_counts getter
    pub fn is_word_enable(&self) -> bool {
        self.word_counts
    }

    /// max_width setter
    pub fn width_toggle(&mut self, enabled: bool) {
        self.max_width = enabled;
    }

    /// max_width getter
    pub fn is_width_enable(&self) -> bool {
        self.max_width
    }

    /// Appends a file to be counted.
    pub fn append_file(&mut self, filename: &str) {
        self.files.push(String::from(filename));
    }

    /// Gets all files to be counted.
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }
}

impl Default for Config {
    /// Returns the default Config.
    ///
    /// Default Config enables to print newline, word, and byte counts
    /// that reads from standard input.
    fn default() -> Self {
        Config {
            byte_counts: true,
            char_counts: false,
            line_counts: true,
            word_counts: true,
            max_width: false,
            files: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c: Config = Default::default();
        assert!(c.byte_counts);
        assert!(c.line_counts);
        assert!(c.word_counts);
        assert!(!c.char_counts);
        assert!(!c.max_width);
        assert_eq!(0, c.files().len());

        c.byte_toggle(false);
        assert!(!c.byte_counts);
        c.char_toggle(true);
        assert!(c.char_counts);
        let expected = "test";
        c.append_file(expected);
        assert_eq!(1, c.files().len());
        assert_eq!(String::from(expected), c.files()[0]);
    }
}
