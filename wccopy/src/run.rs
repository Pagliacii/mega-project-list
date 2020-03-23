use std::default::Default;
use std::error::Error;

use crate::config::Config;
use crate::count::Counter;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut total_bytes = 0;
    let mut total_chars = 0;
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut max_width = 0;

    for file in config.files() {
        // 1. count
        let mut counter: Counter = Default::default();
        counter.set_from(file);
        counter.count()?;
        // 2. print (newline, word, character, byte, maximum line length)
        if config.is_line_enable() {
            total_lines += counter.get_lines();
            print_value(counter.get_lines());
        }
        if config.is_word_enable() {
            total_words += counter.get_words();
            print_value(counter.get_words());
        }
        if config.is_char_enable() {
            total_chars += counter.get_chars();
            print_value(counter.get_chars());
        }
        if config.is_byte_enable() {
            total_bytes += counter.get_bytes();
            print_value(counter.get_bytes());
        }
        if config.is_width_enable() {
            if counter.get_width() > max_width {
                max_width = counter.get_width();
            }
            print_value(counter.get_width());
        }
        if config.files().len() > 1 || file != "-" {
            print!("{}", file);
        }
        println!();
    }

    // 3. summary
    if config.files().len() == 1 {
        return Ok(());
    }
    if config.is_line_enable() {
        print_value(total_lines);
    }
    if config.is_word_enable() {
        print_value(total_words);
    }
    if config.is_char_enable() {
        print_value(total_chars);
    }
    if config.is_byte_enable() {
        print_value(total_bytes);
    }
    if config.is_width_enable() {
        print_value(max_width);
    }
    println!("total");

    Ok(())
}

fn print_value(value: usize) {
    print!("{:>7} ", value);
}
