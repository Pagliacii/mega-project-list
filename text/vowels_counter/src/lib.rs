#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
        assert_eq!(vowels_counter(text), 19);
    }
}

use std::collections::HashMap;

pub fn vowels_counter(text: &str) -> usize {
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let count = counter.entry(c).or_insert(0);
                *count += 1;
            }
            _ => continue,
        }
    }

    let mut count = 0;
    for (key, val) in counter.iter() {
        println!("'{}': {}", key, val);
        count += val;
    }
    count
}
