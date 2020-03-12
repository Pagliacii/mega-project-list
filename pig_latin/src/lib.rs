#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_on_consonant() {
        assert_eq!(pig_latin("Banana"), "anana-Bay");
        assert_eq!(pig_latin("banana"), "anana-bay");
    }

    #[test]
    fn it_works_on_vowel() {
        assert_eq!(pig_latin("apple"), "apple-hay");
        assert_eq!(pig_latin("Apple"), "Apple-hay");
    }

    #[test]
    fn it_works_on_empty_string() {
        assert_eq!(pig_latin(""), "");
    }
}

pub fn pig_latin(input: &str) -> String {
    let mut c = input.trim().chars();
    match c.next() {
        Some(first) => match first.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", input.trim()),
            _ => format!("{}-{}ay", c.as_str(), first),
        },
        None => String::new(),
    }
}
