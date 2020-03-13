#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Controls the behaviors of this program.
///
/// The default behavior is that prints the newline counts, word counts,
/// and character counts to screen. Just like using the `--lines`,
/// `--words`, and `--chars` flags.
///
/// Printing order: newline, word, character, byte, maximum line length
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
