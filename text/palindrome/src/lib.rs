#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_palindrome("racecar"), true);
        assert_eq!(is_palindrome("hello"), false);
    }
}

pub fn is_palindrome(text: &str) -> bool {
    for (fw, bw) in text.chars().zip(text.chars().rev()) {
        if fw != bw {
            return false;
        }
    }
    true
}
