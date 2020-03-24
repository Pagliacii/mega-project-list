#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_ascii_string() {
        let test = "hello, world!";
        let excepted = String::from("!dlrow ,olleh");
        assert_eq!(reverse(test), excepted);
        assert_eq!(reverse_by_iterator(test), excepted);
    }
}

pub fn reverse(source: &str) -> String {
    let len = source.len();
    let mut chars = source.chars();
    let mut result = String::with_capacity(len);

    for _ in 0..len {
        result.insert(0, chars.next().unwrap());
    }
    result
}

pub fn reverse_by_iterator(s: &str) -> String {
    s.chars().rev().collect::<String>()
}
