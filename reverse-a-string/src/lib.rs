#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_ascii_string() {
        let test = String::from("hello, world!");
        let excepted = String::from("!dlrow ,olleh");
        assert_eq!(reverse(test), excepted);
    }
}

pub fn reverse(source: String) -> String {
    let len = source.len();
    let mut chars = source.as_str().chars();
    let mut result = String::with_capacity(len);

    for _ in 0..len {
        result.insert(0, chars.next().unwrap());
    }
    result
}
